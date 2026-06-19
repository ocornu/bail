<p align="center">
  <img src="logo.svg" alt="bail crate logo" width="140" height="140">
</p>

<h1 align="center">bail</h1>

<p align="center">
  <a href="https://crates.io/crates/bail"><img src="https://img.shields.io/crates/v/bail?logo=hackthebox&logoColor=white" alt="crates.io"></a>
  <a href="https://docs.rs/bail"><img src="https://img.shields.io/docsrs/bail?logo=docs.rs" alt="docs.rs"></a>
  <a href="https://github.com/ocornu/bail"><img src="https://img.shields.io/badge/github-repo-blue?logo=github" alt="github.com"></a>
  <br/>
  <img src="https://img.shields.io/badge/no__std-compatible-brightgreen?logo=rust" alt="no_std">
  <img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-brightgreen?logo=unlicense&logoColor=white" alt="MIT/Apache-2.0">
</p>

A tiny, zero-overhead `no_std` Rust crate for cleaner early-exit paths.

Turn verbose error-checking blocks into sleek, fluent method chains using the nifty `?` operator.

## 🛠️ The Problem

Rust functions often start with a wall of boilerplate guard clauses. While effective, they can clutter your code:

```rust
fn process_data(user: &User) -> Result<(), MyError> {
    if user.age < 18 {
        return Err(MyError::TooYoung);
    }

    if user.name.trim().is_empty() {
        return Err(MyError::InvalidName);
    }
    
    // Actual logic goes here...
    Ok(())
}
```

## ✨ The Solution

`bail` adds a lightweight extension trait to `bool`. This lets you write your error conditions as functional expressions that short-circuit instantly.

```rust
use bail::Err;

fn process_data(user: &User) -> Result<(), MyError> {
    (user.age < 18).Err(MyError::TooYoung)?;
    user.name.trim().is_empty().Err(MyError::InvalidName)?;

    // Actual logic goes here...
    Ok(())
}
```

## 📦 Option Support

You can also short-circuit functions that return an `Option` instead of a `Result`:

```rust
use bail::None;

fn find_user(id: u64) -> Option<User> {
    (id == 0).None()?; // Returns early with None if id is 0

    // Actual logic goes here...
    Some(user)
}
```

## ⚙️ How it Works

No macro, pure Rust: the crate injects a single trait implementation into core booleans. Because the function is marked `#[inline(always)]`, the compiler completely optimizes it away. You get gorgeous syntax with absolutely zero runtime performance cost.

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bail = "0.2.2"
```
