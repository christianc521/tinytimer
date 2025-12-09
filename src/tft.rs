use embedded_graphics_framebuf::FrameBuf;
use esp_backtrace as _;
use esp_hal::gpio::Output;
use esp_hal::Async;
use heapless::Vec;
use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use display_interface_spi::SPIInterface;
use ili9341::{DisplaySize240x320, Ili9341, Orientation};
use embedded_graphics::{geometry::{ AnchorPoint, Point }, iterator::raw::RawDataSlice, mono_font::{MonoTextStyle, MonoTextStyleBuilder}, pixelcolor::raw::BigEndian, primitives::{Line, Polyline, PrimitiveStyle, StyledDrawable, Triangle}, text::{renderer::CharacterStyle, Alignment, Baseline, TextStyleBuilder}, pixelcolor::raw::{RawU16}};
use esp_backtrace as _;
use eg_seven_segment::{SevenSegmentStyleBuilder};
use esp_hal::{
    gpio::{GpioPin, Level},
    delay::Delay,
    peripherals::SPI2,
    spi::{
        master::Config as SpiConfig,
        master::Spi,
        Mode as SpiMode
    }
};
use esp_hal::time::RateExtU32;
use embedded_graphics::{
    image::Image,
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle, StrokeAlignment},
    text::Text,
};

use profont::PROFONT_18_POINT;
use tinytga::Tga;
use crate::{animations::{Animation, FrameType}, clock_util::SessionState, draw_panels::{Panel, PanelPosition, Payload}, scenes::SceneManager};

pub type TFTSpiDevice<'spi> = 
    ExclusiveDevice<Spi<'spi, Async>, Output<'spi>, NoDelay>;

pub type TFTSpiInterface<'spi> =
   SPIInterface<
        ExclusiveDevice<Spi<'spi, Async>, Output<'spi>, NoDelay>,
        Output<'spi>
        >;

// NOTE: Display Hardware
pub struct TFT<'spi>
{
    pub display: Ili9341<TFTSpiInterface<'spi>, Output<'spi>>,
    pub playing_animation: bool,
    top_frame_buffer: FrameBuf<Rgb565, [Rgb565; 76800]>,
    scene_manager: SceneManager
}

impl<'spi> TFT<'spi> {
    pub fn new(
        spi2: SPI2,
        sclk: GpioPin<6>,
        miso: GpioPin<5>,
        mosi: GpioPin<7>,
        cs: GpioPin<2>,
        rst: GpioPin<3>,
        dc: GpioPin<4>
        ) -> TFT<'spi> {
        let rst_output = Output::new(rst, Level::Low);
        let dc_output = Output::new(dc, Level::Low);
        let spi = Spi::new(
            spi2, 
            SpiConfig::default()
                .with_frequency(RateExtU32::MHz(40))
                .with_mode(SpiMode::_0))
            .unwrap()
            .with_sck(sclk)
            .with_miso(miso)
            .with_mosi(mosi)
            .into_async();

        let cs_output = Output::new(cs, Level::High);
        let spi_device = ExclusiveDevice::new_no_delay(spi, cs_output).unwrap();
        let interface = SPIInterface::new(spi_device, dc_output);

        let display = Ili9341::new(
            interface, 
            rst_output, 
            &mut Delay::new(), 
            Orientation::Landscape, 
            DisplaySize240x320
        ).unwrap();

        let top_fb = FrameBuf::new_with_origin([Rgb565::WHITE; 320 * 240], 320, 240, Point::new(0, 0));

        TFT { 
            display,
            playing_animation: false,
            top_frame_buffer: top_fb,
            scene_manager: SceneManager::default()
        }
    }
    
    pub fn clear(&mut self, color: Rgb565) {
        self.display.clear(color).unwrap();
    }

    pub fn initialize_scene(&mut self) {
        self.render_divider(SessionState::Working);
        self.render_segmented(&PanelPosition::Bottom, "00:00:00");
    }

    // Match state machine events to draw functions
    pub fn handle_payload(&mut self, panel: &Panel) {
        let frame = &panel.0;
        let payload = &panel.1;
        let state = match frame {
            PanelPosition::Top => SessionState::Working,
            PanelPosition::Bottom => SessionState::Break,
            _ => SessionState::Paused
        };

        match payload {
            Payload::Time(bytes) => {
                let message = if let Ok(text) = str::from_utf8(bytes){
                    text
                } else {
                    "error"
                };
                self.render_segmented(frame, message);
                self.render_divider(state);
            },
            Payload::CursorMove(start, end) => self.animate_cursor(start, end),
            Payload::Animate(animation) => {
                // Index flag for empty space in the queue
                let mut empty_index: Option<usize> = None;

                // Check for space in the SceneManager animation_queue
                for ( index, queued_animation) in 
                    self
                    .scene_manager
                    .animation_queue
                    .iter_mut()
                    .enumerate() {
                    match queued_animation {
                        Animation::Empty => {
                            empty_index = Some(index);
                            break;
                        },
                        _ => ()
                    }
                }

                // Only add the animation to the queue if there's space
                if let Some(index) = empty_index {
                    self.scene_manager.animation_queue[index] = *animation;
                    self.playing_animation = true;
                    self.render_next_frame();
                }
            }
            _ => (),
        }
    }

    pub fn render_next_frame(&mut self) {
        // Grab array of frames to be rendered
        let frame_queue = self.scene_manager.play_next();

        // Empties flag; 
        // if equal to SceneManager animation_queue[] capacity,
        // all animations have been exhausted
        // set tft playing_animation to false
        let mut empty_count: u8 = 0;
        for frame in frame_queue {
            match frame {
                FrameType::Rectangle(rect) => todo!(),
                FrameType::Empty => empty_count += 1
            }
        }

        // Turn off 30 fps render flag if no more frames in the queue
        if empty_count == 6 { self.playing_animation = false };
    }

    fn animate_cursor(&mut self, start: &Point, end: &Point) {
        let initial_data = self.top_frame_buffer.data;
        let area = self.top_frame_buffer.bounding_box();

        let mut cursor = Rectangle::with_corners(Point::zero(), *start);
        let cursor_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(154, 153, 150))
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();

        let route = Line::new(start.clone(), end.clone());

        for point in route.points() {
            // Clear the cursor from the buffer
            self.top_frame_buffer.data = initial_data;

            // Draw display without the cursor
            self.display.fill_contiguous(&area, self.top_frame_buffer.data).unwrap();

            // Move the bottom right corner of the cursor to 
            // a point on the slope (travel route)
            let x = point.x as u32;
            let y = point.y as u32;
            cursor = cursor.resized(Size::new(x, y), AnchorPoint::TopLeft);

            // Update buffer data with a new cursor and position
            let _ = cursor.draw_styled(&cursor_style, &mut self.top_frame_buffer).unwrap();
            
            // Draw the buffer to display with the cursor
            self.display.fill_contiguous(&area, self.top_frame_buffer.data).unwrap();
        }
    }
    
    #[inline]
    pub fn render_segmented(&mut self, frame: &PanelPosition, message: &str) {
        // Set buffer area to the corresponding timer location.
        let ( area, color ) = match frame {
            PanelPosition::Top => {
                ( Rectangle::new(Point::new(30, 20), self.top_frame_buffer.size()), Rgb565::new(123, 191, 255) )
            }
            PanelPosition::Bottom => {
                ( Rectangle::new(Point::new(30, 170), self.top_frame_buffer.size()), Rgb565::new(255, 148, 150) )
            }
            _ => return
        };
        // Reset the buffer to black, but don't draw to the screen yet
        let _ = &mut self.top_frame_buffer.fill_solid(&self.top_frame_buffer.bounding_box(), Rgb565::BLACK).unwrap();

        let style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(30, 50))
            .digit_spacing(10)
            .segment_width(5)
            .segment_color(color)
            .build();

        let center = Point::new(0, 25);
        let text = Text::with_baseline(message, center, style, Baseline::Middle);

        // Write time pixel data to the buffer
        let _ = text.draw(&mut self.top_frame_buffer).unwrap();

        // Finally, draw the buffer to the screen
        let _ = self.display.fill_contiguous(&area, self.top_frame_buffer.data).unwrap();
    }

    #[inline]
    pub fn render_divider(&mut self, mode: SessionState) {
        let mut div_fb = FrameBuf::new_with_origin([Rgb565::BLACK; 320 * 40], 320, 40, Point::new(0, 100));
        let area = Rectangle::new(Point::new(0, 100), div_fb.size());

        let break_divider_points: [Point; 4] = [
            Point::new(70, 30),
            Point::new(160, 30),
            Point::new(190, 10),
            Point::new(290, 10),
        ];

        let working_divider_points: [Point; 4] = [
            Point::new(70, 10),
            Point::new(160, 10),
            Point::new(190, 30),
            Point::new(290, 30),
        ];

        if mode == SessionState::Paused {
            let pause_style = PrimitiveStyleBuilder::new()
                .fill_color(Rgb565::WHITE)
                .build();
            // Left Pause Icon Rectangle
            let _ = Rectangle::new(Point::new(35, 10), Size::new(5, 20))
            .into_styled(pause_style)
            .draw(&mut div_fb)
            .unwrap();
            // Right Pause Icon Rectangle
            let _ = Rectangle::new(Point::new(45, 10), Size::new(5, 20))
            .into_styled(pause_style)
            .draw(&mut div_fb)
            .unwrap();

            let _ = Polyline::new(&working_divider_points)
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 3))
                .draw(&mut div_fb)
                .unwrap();
            let _ = Polyline::new(&break_divider_points)
                .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 3))
                .draw(&mut div_fb)
                .unwrap();

            let text = "paused";
            let text_style = TextStyleBuilder::new()
                .alignment(Alignment::Right)
                .baseline(Baseline::Middle)
                .build();
            let character_style: MonoTextStyle<'_, Rgb565> = MonoTextStyleBuilder::new()
                .font(&PROFONT_18_POINT)
                .text_color(Rgb565::WHITE)
                .build();

            let top_position = Point::new(290, 20);
            Text::with_text_style(
                text,
                top_position,
                character_style,
                text_style)
            .draw(&mut div_fb).unwrap();

            let _ = self.display.fill_contiguous(&area, div_fb.data).unwrap();
            return
        }

        let ( color, running_icon, text, line_points ) = match mode {
            // Light Blue, Pointing Up
            SessionState::Working => { 
                (Rgb565::new(123, 191, 255),
                 Triangle::new(Point::new(25, 30), Point::new(55, 30), Point::new(40, 10)),
                 "working",
                 &working_divider_points)
            },
            // Salmon Pink, Pointing Down
            SessionState::Break => { 
                (Rgb565::new(255, 148, 150),
                 Triangle::new(Point::new(25, 10), Point::new(55, 10), Point::new(40, 30)),
                 "on break",
                 &break_divider_points)
            },
            SessionState::Paused => { 
                (Rgb565::WHITE,
                 Triangle::new(Point::new(25, 30), Point::new(55, 30), Point::new(40, 10)),
                 "paused",
                 &working_divider_points)
            },
        };

        // Draw arrow/pause icon to buffer
        let _ = running_icon.draw_styled(&PrimitiveStyle::with_fill(color), &mut div_fb).unwrap();

        // Draw divider to buffer
        let _ = Polyline::new(line_points)
            .into_styled(PrimitiveStyle::with_stroke(color, 3))
            .draw(&mut div_fb)
            .unwrap();

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Right)
            .baseline(Baseline::Middle)
            .build();
        let character_style: MonoTextStyle<'_, Rgb565> = MonoTextStyleBuilder::new()
            .font(&PROFONT_18_POINT)
            .text_color(color)
            .build();

        let text_position = Point::new(290, 20);
        Text::with_text_style(
            text,
            text_position,
            character_style,
            text_style)
        .draw(&mut div_fb).unwrap();

        // Draw buffer to display
        let _ = self.display.fill_contiguous(&area, div_fb.data).unwrap();
    }

    pub fn draw_image(&mut self) {
        let data = include_bytes!("../src/assets/background-white.tga");
        let tga: Tga<Rgb565> = Tga::from_slice(data).unwrap();
        let image = Image::with_center(&tga, self.display.bounding_box().center());
        image.draw(&mut self.display).unwrap();
    }
}
