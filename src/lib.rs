//! # bail
//!
//! A tiny, zero-overhead `no_std` Rust crate for cleaner early-exit paths.
//!
//! Turn verbose error-checking blocks into sleek, fluent method chains using the nifty `?` operator.
//!
//! ## 🛠️ The Problem
//!
//! Rust functions often start with a wall of boilerplate guard clauses.
//!
//! While effective, they can clutter your code:
//! ```rust,ignore
//! fn process_data(user: &User) -> Result<(), MyError> {
//!     if user.age < 18 {
//!         return Err(MyError::TooYoung);
//!     }
//!
//!     if user.name.trim().is_empty() {
//!         return Err(MyError::InvalidName);
//!     }
//!
//!     // Actual logic goes here...
//!     Ok(())
//! }
//! ```
//!
//! ## ✨ The Solution
//!
//! `bail` adds a lightweight extension trait to `bool`.
//!
//! This lets you write your error conditions as functional expressions that short-circuit instantly.
//! ```rust,ignore
//! use bail::Err;
//!
//! fn process_data(user: &User) -> Result<(), MyError> {
//!     (user.age < 18).Err(MyError::TooYoung)?;
//!     user.name.trim().is_empty().Err(MyError::InvalidName)?;
//!
//!     // Actual logic goes here...
//!     Ok(())
//! }
//! ```
//!
//! ## 📦 Option Support
//!
//! You can also short-circuit functions that return an `Option` instead of a `Result`:
//!
//! ```rust,ignore
//! use bail::None;
//!
//! fn find_user(id: u64) -> Option<User> {
//!     (id == 0).None()?; // Returns early with None if id is 0
//!
//!     // Actual logic goes here...
//!     Some(user)
//! }
//! ```
#![no_std]

mod option;
mod result;

pub use option::None;
pub use result::Err;
