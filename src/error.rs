use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum Error {
    #[snafu(display("Failed to deserialize a value from rkyv file {file_name}"))]
    Rkyv {
        source: rkyv::rancor::Error,
        file_name: String,
    },
    #[snafu(display("{message}"))]
    IoOperation {
        message: String,
        source: std::io::Error,
    },
}
