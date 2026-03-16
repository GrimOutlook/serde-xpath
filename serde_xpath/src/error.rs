use std::fmt;

#[derive(Debug)]
pub enum Error {
    XmlParse(roxmltree::Error),
    XPath(String),
    MissingField(String),
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::XmlParse(e) => write!(f, "XML parse error: {}", e),
            Error::XPath(msg) => write!(f, "XPath error: {}", msg),
            Error::MissingField(field) => write!(f, "missing field: {}", field),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::XmlParse(e) => Some(e),
            _ => None,
        }
    }
}

impl serde::de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl From<roxmltree::Error> for Error {
    fn from(e: roxmltree::Error) -> Self {
        Error::XmlParse(e)
    }
}
