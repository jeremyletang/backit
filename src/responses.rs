// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use iron::IronResult;
use iron::Response as IronResponse;
use iron::status::Status;
use json::Error;
use std::convert::Into;

pub fn new<S>(status: Status, res: S)
    -> IronResult<IronResponse> where S: Into<String> {
    let r: String = res.into();
    Ok(IronResponse::with((status, r)))
}

pub fn no_content() -> IronResult<IronResponse> {
    Ok(IronResponse::with((Status::NoContent, "")))
}

pub fn ok<S>(res: S)
    -> IronResult<IronResponse> where S: Into<String> {
    let r: String = res.into();
    Ok(IronResponse::with((Status::Ok, r)))
}

pub fn bad_request<S>(err: S)
    -> IronResult<IronResponse> where S: Into<String> {
    let e = Error::bad_request(err);
    Ok(IronResponse::with((e.status(), e.as_json())))
}

pub fn internal_error<S>(err: S)
    -> IronResult<IronResponse> where S: Into<String> {
    let e = Error::internal_error(err);
    Ok(IronResponse::with((e.status(), e.as_json())))
}

pub fn not_found<S>(err: S)
    -> IronResult<IronResponse> where S: Into<String> {
    let e = Error::not_found(err);
    Ok(IronResponse::with((e.status(), e.as_json())))
}

pub fn unauthorized<S>(err: S)
    -> IronResult<IronResponse> where S: Into<String> {
    let e = Error::unauthorized(err);
    Ok(IronResponse::with((e.status(), e.as_json())))
}
