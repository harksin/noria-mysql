#![feature(box_syntax, box_patterns)]
#![feature(nll)]
#![feature(try_from)]

extern crate api as distributary;
extern crate arccstr;
extern crate chrono;
extern crate failure;
extern crate msql_srv;
extern crate nom_sql;
extern crate regex;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;

mod convert;
mod referred_tables;
mod rewrite;
mod schema;
mod soup_backend;
mod utils;

pub use schema::Schema;
pub use soup_backend::SoupBackend;
