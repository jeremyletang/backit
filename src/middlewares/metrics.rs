// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use iron::prelude::*;
use time::precise_time_ns;
use iron::{AfterMiddleware, BeforeMiddleware, typemap};

pub struct MetricsMid;

impl typemap::Key for MetricsMid {
    type Value = u64;
}

impl BeforeMiddleware for MetricsMid {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<MetricsMid>(precise_time_ns());
        Ok(())
    }
}

fn make_path(path: &Vec<&str>) -> String {
    path.iter().fold("".to_string(), |mut acc, ref x| {
        acc.push_str("/");
        acc.push_str(&*x);
        acc
    })
}

impl AfterMiddleware for MetricsMid {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() -
                    *req.extensions
            .get::<MetricsMid>()
            .expect("cannot get response time middleware from the context");
        let status = res.status
            .clone()
            .expect("cannot get status from response");
        info!("request from {} to {} {} - {} in {} ms",
              req.remote_addr,
              req.method,
              make_path(&req.url.path()),
              status,
              (delta as f64) / 1000000.0);
        Ok(res)
    }
}
