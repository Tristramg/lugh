use diesel::ConnectionError as DieselConnectionError;
use diesel::result::Error as DieselResultError;
use iron::prelude::*;
use iron::status;
use std::error::Error;
use std::fmt::{self, Debug};
use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct BadRequestError(pub String);

impl fmt::Display for BadRequestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for BadRequestError {
    fn description(&self) -> &str { &*self.0.as_str() }
}

impl From<BadRequestError> for IronError {
    fn from(error: BadRequestError) -> IronError {
        IronError::new(error, status::BadRequest)
    }
}

// FIXME: Remove this generic error once all errors are implemented
#[derive(Debug)]
pub struct StringError(pub &'static str);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}

impl From<DieselConnectionError> for StringError {
    fn from(_: DieselConnectionError) -> StringError {
        StringError("Database connection error")
    }
}

impl From<DieselResultError> for StringError {
    fn from(_: DieselResultError) -> StringError {
        StringError("ORM result error")
    }
}

impl From<FromUtf8Error> for StringError {
    fn from(_: FromUtf8Error) -> StringError {
        StringError("Error when parsing bytes to String")
    }
}

impl From<StringError> for IronError {
    fn from(error: StringError) -> IronError {
        IronError::new(error, status::InternalServerError)
    }
}