# random_password_generator
Rust library to generate random passwords, you can define whether the password will have alphabetical, numeric or special characters.

## Installation

To use `random_password_generator` in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
random_password_generator = "0.1.0"

```rust
use random_password_generator::generate_password;

fn main() {
    let random_letter = generate_password(true, true,true, 5);
    println!("new password generated: {}", random_letter)
}
```
