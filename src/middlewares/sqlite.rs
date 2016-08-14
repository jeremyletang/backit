// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use diesel::sqlite::SqliteConnection;
use iron::{Request, IronResult};
use iron::{BeforeMiddleware, typemap};
use r2d2_diesel::ConnectionManager;
use r2d2;
use std::convert::Into;

pub struct SqliteConnectionMid {
    pool: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}

impl SqliteConnectionMid {
    pub fn new<S>(database_url: S) -> SqliteConnectionMid where S: Into<String> {
        let database_url: String = database_url.into();
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<SqliteConnection>::new(&*database_url);
        let pool = r2d2::Pool::new(config, manager)
            .expect(&format!("Error connecting to {}", database_url));
        SqliteConnectionMid { pool: Arc::new(pool) }
    }
}

impl typemap::Key for SqliteConnectionMid {
    type Value = Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>;
}

impl BeforeMiddleware for SqliteConnectionMid {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let pool = self.pool.clone();
        req.extensions.insert::<SqliteConnectionMid>(pool);
        Ok(())
    }
}

pub fn extract_connection_from_request(req: &mut Request)
                                       -> Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>> {
    let pool = req.extensions
        .get::<SqliteConnectionMid>()
        .expect("cannot get database connection pool from context");
    pool.clone()
}
