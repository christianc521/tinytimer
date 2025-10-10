use core::convert::Infallible;
use derive_more::derive::{Display, Error, From};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display("_0:?")]
    TaskSpawn(#[error(not(source))] embassy_executor::SpawnError),

    #[display("Error setting state")]
    SetStateError
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::SetStateError
    }
}
