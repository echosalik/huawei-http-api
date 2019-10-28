use failure::Fail;

#[derive(Debug, Fail)]
pub enum Errors {
  #[fail(display = "{}", _0)]
  HttpError(#[cause] reqwest::Error),
  #[fail(display = "{}", _0)]
  DotEnv(#[cause] dotenv::Error),
  #[fail(display = "{}", _0)]
  XmlError(#[cause] quick_xml::Error),
  #[fail(display = "{}", _0)]
  Utf8Error(#[cause] std::string::FromUtf8Error),
  #[fail(display = "{}", _0)]
  Infallible(#[cause] std::convert::Infallible),
  #[fail(display = "{}", _0)]
  SerdeXmlError(#[cause] failure::SyncFailure<serde_xml_rs::Error>),
  #[fail(display = "{}", _0)]
  Base64(#[cause] base64::DecodeError),
  #[fail(display = "{}", _0)]
  SerdeJSON(#[cause] serde_json::error::Error),
  #[fail(display = "{}", _0)]
  InvalidHeaderValue(#[cause] reqwest::header::InvalidHeaderValue),
  #[fail(display = "{}", _0)]
  InvalidHeaderName(#[cause] reqwest::header::InvalidHeaderName),
}

impl From<dotenv::Error> for Errors {
  fn from(error: dotenv::Error) -> Errors {
    return Errors::DotEnv(error);
  }
}

impl From<reqwest::Error> for Errors {
  fn from(error: reqwest::Error) -> Errors {
    return Errors::HttpError(error);
  }
}

impl From<quick_xml::Error> for Errors {
  fn from(error: quick_xml::Error) -> Errors {
    return Errors::XmlError(error);
  }
}

impl From<std::string::FromUtf8Error> for Errors {
  fn from(error: std::string::FromUtf8Error) -> Errors {
    return Errors::Utf8Error(error);
  }
}

impl From<std::convert::Infallible> for Errors {
  fn from(error: std::convert::Infallible) -> Errors {
    return Errors::Infallible(error);
  }
}

impl From<serde_xml_rs::Error> for Errors {
  fn from(error: serde_xml_rs::Error) -> Errors {
    let x = failure::SyncFailure::<serde_xml_rs::Error>::new(error);
    return Errors::SerdeXmlError(x);
  }
}

impl From<base64::DecodeError> for Errors {
  fn from(error: base64::DecodeError) -> Errors {
    return Errors::Base64(error);
  }
}

impl From<serde_json::Error> for Errors {
  fn from(error: serde_json::Error) -> Errors {
    return Errors::SerdeJSON(error);
  }
}

impl From<reqwest::header::InvalidHeaderValue> for Errors {
  fn from(error: reqwest::header::InvalidHeaderValue) -> Errors {
    return Errors::InvalidHeaderValue(error);
  }
}

impl From<reqwest::header::InvalidHeaderName> for Errors {
  fn from(error: reqwest::header::InvalidHeaderName) -> Errors {
    return Errors::InvalidHeaderName(error);
  }
}
