<p align="center">
  <img src="logo.svg" alt="bail crate logo" width="140" height="140">
</p>

<h1 align="center">bail</h1>

<p align="center">
  <a href="https://crates.io/crates/bail"><img src="https://shields.io" alt="Crates.io"></a>
  <a href="https://docs.rs/bail"><img src="https://docs.rs/badge.svg" alt="Docs.rs"></a>
  <img src="https://shields.io" alt="no_std compatible">
</p>

# bail

A tiny, zero-overhead `no_std` Rust crate for cleaner early-exit paths.

Turn verbose error-checking blocks into sleek, fluent method chains using the `?` operator.

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

```rust
pub trait Err<E> {
    fn Err(&self, error: E) -> Result<(), E>;
}

impl<E> Err<E> for bool {
    #[inline(always)]
    fn Err(&self, error: E) -> Result<bool, E> {
        if *self { Err(error) } else { Ok(()) }
    }
}
```

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bail = "0.2.2"
```
