//! Data storage module using SQLite
//! 
//! Handles historical data storage with 7-day TTL.

pub mod database;

pub use database::Database;
