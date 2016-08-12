// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use iron::request::Body;
use self::error::Error;
use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Read;

pub mod error;

pub fn from_body<T>(body: &mut Body) -> Result<T, Error>
    where T: Serialize + Deserialize + Default
{
    let mut s = String::new();
    match body.read_to_string(&mut s) {
        Ok(_) => {}
        Err(e) => {
            let err = format!("cannot read request body {}", e);
            return Err(Error::internal_error(&err));
        }
    }

    let t: T = match serde_json::from_str(&s) {
        Ok(g) => g,
        Err(_) => {
            let err = format!("expected type: {}", serde_json::to_string(&T::default()).ok().unwrap());
            return Err(Error::bad_request(&err));
        }
    };

    Ok(t)
}
