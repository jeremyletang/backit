// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::cors::CorsMid;
pub use self::metrics::MetricsMid;
pub use self::sqlite::{SqliteConnectionMid, extract_sqlite_from_request};
pub use self::postgres::{PostgresConnectionMid, extract_postgres_from_request};

mod cors;
mod metrics;
mod postgres;
mod sqlite;
