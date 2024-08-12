mod mysql;
mod postgres;
mod presentation;
pub mod model;

pub(self) use mysql::MySQL;
pub(self) use postgres::PostgreSQL;
pub(crate) use presentation::Persistence;