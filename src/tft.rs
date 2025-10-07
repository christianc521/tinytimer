use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use esp_backtrace as _;
use esp_hal::gpio::Output;
use esp_hal::Async;

use embedded_hal_bus::spi::{ExclusiveDevice, NoDelay};
use display_interface_spi::SPIInterface;
use ili9341::{DisplaySize240x320, Orientation, Ili9341};
use esp_backtrace as _;
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
    mono_font::{MonoTextStyle},
    image::Image,
    prelude::*,
    text::{Alignment, Text},
    pixelcolor::Rgb565,
    primitives::{ PrimitiveStyleBuilder, Rectangle, StrokeAlignment}
};
use profont::{ PROFONT_24_POINT, PROFONT_18_POINT };
use tinytga::Tga;

pub type TFTSpiDevice<'spi> = 
    ExclusiveDevice<Spi<'spi, Async>, Output<'spi>, NoDelay>;

pub type TFTSpiInterface<'spi> =
   SPIInterface<
        ExclusiveDevice<Spi<'spi, Async>, Output<'spi>, NoDelay>,
        Output<'spi>
        >;


#[derive(Clone, Copy)]
pub struct DoubleTimerRectangles {
    top: Rectangle,
    middle: Rectangle,
    bottom_left: Rectangle,
    bottom_right: Rectangle
}

// NOTE: Display Hardware
pub struct TFT<'spi>
{
    pub display: Ili9341<TFTSpiInterface<'spi>, Output<'spi>>,
    pub layout_panels: DoubleTimerRectangles
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

        let top_panel = 
                Rectangle::new(Point::new(0, 0), Size::new(320, 64));

        let middle_panel = 
                Rectangle::new(Point::new(0, 64), Size::new(320, 96));

        let bottom_left_panel = 
                Rectangle::new(Point::new(0, 160), Size::new(160, 80));
        let bottom_right_panel = 
                Rectangle::new(Point::new(160, 160), Size::new(160, 80));

        let layout_panels = DoubleTimerRectangles {
            top: top_panel,
            middle: middle_panel, 
            bottom_left: bottom_left_panel, 
            bottom_right: bottom_right_panel
        };

        TFT { 
            display,
            layout_panels
        }
    }
    
    pub fn clear(&mut self, color: Rgb565) {
        self.display.clear(color).unwrap();
    }

    pub fn render_border(&mut self) {
        let style = PrimitiveStyleBuilder::new()
            .stroke_width(2)
            .stroke_alignment(StrokeAlignment::Center)
            .stroke_color(Rgb565::WHITE)
            .build();

        self.layout_panels.middle
            .into_styled(style)
            .draw(&mut self.display)
            .unwrap();

        self.layout_panels.top
            .into_styled(style)
            .draw(&mut self.display)
            .unwrap();
        self.layout_panels.bottom_left
            .into_styled(style)
            .draw(&mut self.display)
            .unwrap();

        self.layout_panels.bottom_right
            .into_styled(style)
            .draw(&mut self.display)
            .unwrap();
    }
    
    pub fn draw_focus_time(&mut self, message: &str) {
        let mut panel = self.display.clipped(&self.layout_panels.middle);
        let center = panel.bounding_box().center();

        let style = MonoTextStyle::new(&PROFONT_24_POINT, Rgb565::WHITE);
        
        let _ = Text::with_alignment(message, center, style, Alignment::Center)
                .draw(&mut panel).unwrap();
    }

    pub fn draw_unfocused_time(&mut self, message: &str) {
        let mut panel = self.display.clipped(&self.layout_panels.bottom_right);
        let center = panel.bounding_box().center();

        let style = MonoTextStyle::new(&PROFONT_18_POINT, Rgb565::WHITE);

        let _ = Text::with_alignment(message, center, style, Alignment::Center)
                .draw(&mut panel).unwrap();
    }

    pub fn draw_image(&mut self) {
        let data = include_bytes!("../src/assets/meowl-new.tga");
        let tga: Tga<Rgb565> = Tga::from_slice(data).unwrap();
        let image = Image::with_center(&tga, self.display.bounding_box().center());
        image.draw(&mut self.display).unwrap();
    }
}


