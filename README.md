# paseto_maker

This library provides high-level functionality for creating, handling, and managing PASETO tokens.

**Note:** This crate is currently in Alpha. The API is subject to change and may contain bugs.

# Overview

This library includes modules for defining claims, handling errors, and creating/verifying PASETO tokens.
It leverages the `rusty_paseto` crate and currently supports PASETO Tokens V4.public.

# Modules

- `claims`: Defines the structure and behavior of the claims that can be embedded in a PASETO token.
- `errors`: Provides error types and handling mechanisms for the library.
- `maker`: Contains the logic for creating and verifying PASETO tokens.

# Re-exports

- `Claims`: The struct representing the claims in a PASETO token.
- `Maker`: The struct used for creating and verifying PASETO tokens.

# Usage Example

```rust
use paseto_maker::{Maker, Claims, version::V4, purpose::Public};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maker = Maker::new_with_keypair().unwrap();
    let claims = Claims::new().with_subject("example");
    let token = maker.create_token(&claims).unwrap();
    println!("Token: {}", token);

    let verified_claims = maker.verify_token(&token)?;
    println!("Verified Claims: {:?}", verified_claims);
    Ok(())
}
```

The `claims` module defines the structure and behavior of the claims that can be embedded in a PASETO token.
The `errors` module provides error types and handling mechanisms for the library.
The `maker` module contains the logic for creating and verifying PASETO tokens.

The `Claims` struct and `Maker` struct are re-exported for ease of use.

This library uses the `rusty_paseto` crate underneath and currently only supports PASETO Tokens V4.public.
