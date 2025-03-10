pub type Result<T> = core::result::Result<T, Error>;

pub struct Error {
    inner: Box<ErrorKind>
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            inner: Box::new(kind)
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::error::Error for Error {}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self::new(kind)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::new(ErrorKind::StdIoError(e))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::new(ErrorKind::PraseIntError(e))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerdeJsonError(e))
    }
}

impl From<osynic_osudb::error::Error> for Error {
    fn from(e: osynic_osudb::error::Error) -> Self {
        Error::new(ErrorKind::OsynicOsudbError(e))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::new(ErrorKind::OsynicSerializerError(e))
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::new(ErrorKind::OsynicSerializerError(e.to_string()))
    }
}

pub enum ErrorKind {
    OsynicSerializerError(String),
    OsynicOsudbError(osynic_osudb::error::Error),
    PraseIntError(std::num::ParseIntError),
    SerdeJsonError(serde_json::Error),
    StdIoError(std::io::Error),
}


impl std::fmt::Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicSerializerError(e) => write!(f, "{:?}", e),
            ErrorKind::OsynicOsudbError(e) => write!(f, "{:?}", e),
            ErrorKind::PraseIntError(e) => write!(f, "{:?}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "{:?}", e),
            ErrorKind::StdIoError(e) => write!(f, "{:?}", e),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OsynicSerializerError(e) => write!(f, "{}", e),
            ErrorKind::OsynicOsudbError(e) => write!(f, "{}", e),
            ErrorKind::PraseIntError(e) => write!(f, "{}", e),
            ErrorKind::SerdeJsonError(e) => write!(f, "{}", e),
            ErrorKind::StdIoError(e) => write!(f, "{}", e),
        }
    }
}

