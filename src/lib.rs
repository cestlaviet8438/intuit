//! # inTUIt!
//!
//! *How many puns with this word can I make today...*
//!
//! A terminal UI (TUI) framework for building full-fledged terminal
//! applications.

#![deny(unused_must_use, unused_imports, rust_2018_idioms)]
#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![allow(
	clippy::missing_errors_doc,
	clippy::missing_panics_doc,
	clippy::module_name_repetitions,
	clippy::cast_possible_truncation,
	clippy::cast_possible_wrap,
	unused_imports
)]

mod services;

/// Result type for the entire crate. Uses [`color_eyre`]'s
/// [Result](color_eyre::eyre::Result) type.
pub type Result<T, E = color_eyre::eyre::Report> = color_eyre::eyre::Result<T, E>;
