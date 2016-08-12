// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use iron::status::Status;
use serde::{Serialize, Serializer};
use serde_json;
use std::error::Error as StdError;
use std::fmt;

#[derive(Display, Debug, Eq, PartialEq, Clone)]
pub struct StatusWrapper(Status);

impl Serialize for StatusWrapper {
    fn serialize<S>(&self, s: &mut S) -> Result<(), S::Error> where S: Serializer {
        let &StatusWrapper(status) = self;
        s.serialize_str(&format!("{}", status))
    }
}

#[derive(Display, Debug, Eq, PartialEq, Clone, Serialize)]
pub struct Error {
    pub status: StatusWrapper,
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&*self.to_json())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &*self.message
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl Error {
    pub fn bad_request(message: &str) -> Error {
        Error {
            status: StatusWrapper(Status::BadRequest),
            message: message.to_string(),
        }
    }

    pub fn internal_error(message: &str) -> Error {
        Error {
            status: StatusWrapper(Status::InternalServerError),
            message: message.to_string(),
        }
    }

    pub fn not_found(message: &str) -> Error {
        Error {
            status: StatusWrapper(Status::NotFound),
            message: message.to_string(),
        }
    }

    pub fn unauthorized(message: &str) -> Error {
        Error {
            status: StatusWrapper(Status::Unauthorized),
            message: message.to_string(),
        }
    }

    pub fn code(&self) -> Status {
        let StatusWrapper(c) = self.status;
        return c;
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
