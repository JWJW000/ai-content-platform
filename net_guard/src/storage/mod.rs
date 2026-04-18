//! Data storage module using SQLite
//! 
//! Handles historical data storage with 7-day TTL.

mod database;

pub use database::Database;
