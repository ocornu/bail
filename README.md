# bail

A tiny, zero-overhead Rust crate for cleaner early exit paths.

Turn verbose structural error-checking blocks into sleek, fluent method chains using the `?` operator.

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

## ⚙️ How it Works

The crate injects a single trait implementation into core booleans. Because the function is marked `#[inline(always)]`, the compiler completely optimizes it away. You get gorgeous syntax with absolutely zero runtime performance cost.

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
bail = "0.1.1"
```
