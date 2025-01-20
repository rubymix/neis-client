use hyper::Error as HyperError;
use hyper_util::client::legacy::Error as ClientError;
use serde_json::Error as JsonError;

#[derive(Debug)]
pub enum Error {
    Hyper(HyperError),
    Client(ClientError),
    Json(JsonError),
    Unknown(String),
}

impl Error {
    pub fn new_unknown(message: &str) -> Error {
        Error::Unknown(message.to_owned())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::Hyper(ref err) => err.fmt(f),
            Error::Client(ref err) => err.fmt(f),
            Error::Json(ref err) => err.fmt(f),
            Error::Unknown(ref err) => err.fmt(f),
        }
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::Hyper(err)
    }
}

impl From<ClientError> for Error {
    fn from(err: ClientError) -> Error {
        Error::Client(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Error {
        Error::Json(err)
    }
}
