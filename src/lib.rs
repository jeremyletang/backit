// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(custom_derive, proc_macro)]
#![allow(unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate iron;
#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time as std_time;
extern crate unicase;

pub mod middlewares;
pub mod json;
pub mod macros;
pub mod responses;
pub mod time;
