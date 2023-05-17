#![warn(rust_2018_idioms)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

//! This application provides proxy sorting functionality with multiple DBs as sources.

/// Responsible for DB connecting and proxy selection.
pub mod selector;

/// Models for objects.
pub mod types;
