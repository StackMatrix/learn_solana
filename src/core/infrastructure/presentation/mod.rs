mod mysql;
mod postgres;
mod presentation;
pub mod model;
pub mod repository;

pub(self) use mysql::MySQL;
pub(self) use postgres::PostgreSQL;
pub(crate) use presentation::Persistence;