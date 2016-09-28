// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::sync::Arc;

use diesel::pg::PgConnection;
use iron::{Request, IronResult};
use iron::{BeforeMiddleware, typemap};
use r2d2_diesel::ConnectionManager;
use r2d2;
use std::convert::Into;

pub struct PostgresConnectionMid {
    pool: Arc<r2d2::Pool<ConnectionManager<PgConnection>>>,
}

impl PostgresConnectionMid {
    pub fn new<S>(database_url: S) -> PostgresConnectionMid where S: Into<String> {
        let database_url: String = database_url.into();
        let config = r2d2::Config::builder()
            .pool_size(5)
            .build();
        let manager = ConnectionManager::<PgConnection>::new(&*database_url);
        let pool = r2d2::Pool::new(config, manager)
            .expect(&format!("Error connecting to {}", database_url));
        PostgresConnectionMid { pool: Arc::new(pool) }
    }
}

impl typemap::Key for PostgresConnectionMid {
    type Value = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;
}

impl BeforeMiddleware for PostgresConnectionMid {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let pool = self.pool.clone();
        req.extensions.insert::<PostgresConnectionMid>(pool);
        Ok(())
    }
}

pub fn extract_postgres_from_request(req: &mut Request)
                                     -> Arc<r2d2::Pool<ConnectionManager<PgConnection>>> {
    let pool = req.extensions
        .get::<PostgresConnectionMid>()
        .expect("cannot get database connection pool from context");
    pool.clone()
}

