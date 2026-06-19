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

#[allow(unused)]
pub trait Err<E> {
    #[allow(non_snake_case)]
    fn Err(&self, error: E) -> Result<(), E>;
}

impl<E> Err<E> for bool {
    #[inline(always)]
    fn Err(&self, error: E) -> Result<(), E> {
        if *self { Err(error) } else { Ok(()) }
    }
}

#[allow(unused)]
pub trait None {
    #[allow(non_snake_case)]
    fn None(&self) -> Option<()>;
}

impl None for bool {
    #[inline(always)]
    fn None(&self) -> Option<()> {
        if *self { None } else { Some(()) }
    }
}

#[cfg(test)]
mod err_tests {
    use super::*;

    // A dummy custom error type to ensure the trait works with non-primitive types
    #[derive(Debug, PartialEq, Eq)]
    struct CustomError(&'static str);

    #[test]
    fn test_err_when_true() {
        let condition = true;
        let result = condition.Err(CustomError("Something went wrong"));

        // When true, it must return the Err variant holding our error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), CustomError("Something went wrong"));
    }

    #[test]
    fn test_ok_when_false() {
        let condition = false;
        // Specifying the error type via generic parameter so type inference succeeds
        let result = condition.Err(CustomError("No error should happen"));

        // When false, it must return Ok(())
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ());
    }

    // A helper function simulating real-world usage of the ? operator
    fn check_age(age: u32) -> Result<&'static str, &'static str> {
        // If true, this returns early with Err("Too young")
        (age < 18).Err("Too young")?;

        // If the above didn't exit early, execution continues normally
        Ok("Access granted")
    }

    #[test]
    fn test_early_exit_control_flow() {
        // Test early exit path triggering
        let under_age_result = check_age(16);
        assert_eq!(under_age_result, Err("Too young"));

        // Test normal path continuation
        let adult_result = check_age(21);
        assert_eq!(adult_result, Ok("Access granted"));
    }

    #[test]
    fn test_inline_expression_usage() {
        // Verifies that inline logic works seamlessly without creating a variable first
        let x = 10;
        let res: Result<(), &str> = (x > 5).Err("x is too big");
        assert_eq!(res, Err("x is too big"));
    }
}

#[cfg(test)]
mod none_tests {
    use super::*;

    #[test]
    fn test_none_when_true() {
        let condition = true;
        let result = condition.None();

        // When true, it must return the None variant
        assert!(result.is_none());
    }

    #[test]
    fn test_some_when_false() {
        let condition = false;
        let result = condition.None();

        // When false, it must return Some(())
        assert!(result.is_some());
        assert_eq!(result.unwrap(), ());
    }

    // A helper function simulating real-world usage of ? with Option
    fn find_item(id: u32) -> Option<&'static str> {
        // If true, this returns early from the function with None
        (id == 0).None()?;

        // If execution continues, return the valid item
        Some("Valid Item")
    }

    #[test]
    fn test_option_early_exit_control_flow() {
        // Test early exit path for Option
        let invalid_item = find_item(0);
        assert_eq!(invalid_item, None);

        // Test normal path continuation for Option
        let valid_item = find_item(42);
        assert_eq!(valid_item, Some("Valid Item"));
    }
}
