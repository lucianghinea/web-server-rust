pub enum Error {
    ParsingError,
    UTF8Error(std::string::FromUtf8Error),
    IOError(std::io::Error)
}

impl From<std::io::Error> for Error {
    fn from(internal_err: std::io::Error) -> Self {
        Error::IOError(internal_err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(internal_err: std::string::FromUtf8Error) -> Self {
        Error::UTF8Error(internal_err)
    }
}