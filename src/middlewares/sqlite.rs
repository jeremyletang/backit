// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::sync::Arc;

use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use iron::prelude::*;
use iron::{BeforeMiddleware, typemap};
use r2d2_diesel::ConnectionManager;
use r2d2;

pub struct SqliteConnectionMiddleware {
    pool: Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>,
}

impl SqliteConnectionMiddleware {
    pub fn new() -> SqliteConnectionMiddleware {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let config = r2d2::Config::default();
        let manager = ConnectionManager::<SqliteConnection>::new(&*database_url);
        let pool = r2d2::Pool::new(config, manager)
            .expect(&format!("Error connecting to {}", database_url));
        SqliteConnectionMiddleware { pool: Arc::new(pool) }
    }
}


impl typemap::Key for SqliteConnectionMiddleware {
    type Value = Arc<r2d2::Pool<ConnectionManager<SqliteConnection>>>;
}

impl BeforeMiddleware for SqliteConnectionMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let pool = self.pool.clone();
        req.extensions.insert::<SqliteConnectionMiddleware>(pool);
        Ok(())
    }
}
