# paseto_maker

This library provides high-level functionality for creating, handling, and managing PASETO tokens.

**Note:** This crate is currently in Alpha. The API is subject to change and may contain bugs.

# Overview

This library includes modules for defining claims, handling errors, and creating/verifying PASETO tokens.
It leverages the `rusty_paseto` crate and currently supports PASETO Tokens V4.public.

# Usage Example

```rust
use paseto_maker::{Maker, Claims, version::V4, purpose::Public};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (priv_key, _public_key) = Maker::new_keypair();
    let maker = Maker::new(&priv_key).expect("failed to create maker");
    let claims = Claims::new().with_subject("example");
    let token = maker.create_token(&claims).unwrap();
    println!("Token: {}", token);

    let verified_claims = maker.verify_token(&token)?;
    println!("Verified Claims: {:?}", verified_claims);
    Ok(())
}
```

This library uses the `rusty_paseto` crate underneath and currently only supports PASETO Tokens V4.public.
