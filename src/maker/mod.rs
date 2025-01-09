#![allow(dead_code)]
use std::marker::PhantomData;

use rusty_paseto::{
    core::{
        Key, PasetoAsymmetricPrivateKey, PasetoAsymmetricPublicKey, Public as pPublic, V4 as pV4,
    },
    prelude::{
        AudienceClaim, CustomClaim, ExpirationClaim, IssuedAtClaim, IssuerClaim, NotBeforeClaim,
        PasetoBuilder, SubjectClaim, TokenIdentifierClaim,
    },
};

use crate::{claims::reserved, purpose::Public, version::V4, Claims};
use crate::{
    errors::{MakerError, TokenError},
    purpose::Purpose,
    version::Version,
};
// pub mod error;

pub struct Maker<V: Version, P: Purpose> {
    private_key: Key<64>,
    public_key: Key<32>,
    public_key_bytes: [u8; 32],
    version: String,
    purpose: String,
    _version: PhantomData<V>,
    _purpose: PhantomData<P>,
}

/// `Maker` is a struct that provides functionality to create and manage PASETO (Platform-Agnostic Security Tokens) tokens.
///
/// # Methods
///
/// - `new(private_key: &[u8; 64]) -> Self`
///   - Creates a new `Maker` instance with the given private and public keys.
/// - `new_keypair() -> ([u8; 64], [u8; 32])`
///   - Generates a new Ed25519 keypair and returns the private and public keys.
/// - `create_token(&self, claims: &Claims) -> Result<String, TokenError>`
///   - Creates a new PASETO token with the given claims. Returns the token as a `String` or an error if the token creation fails.
/// - `verify_token(&self, token: &str) -> Result<Claims, TokenError>`
///   - Verifies a PASETO token. Returns the containing Claims or an error if the token verification fails.
///
/// # Example
///
/// ```rust
/// use paseto_maker::{Maker, Claims, version::V4, purpose::Public};
/// let (priv_key, _) = Maker::new_keypair();
/// let maker = Maker::new(&priv_key).expect("failed to create maker");
/// let claims = Claims::new();
/// let token = maker.create_token(&claims).unwrap();
/// ```
impl Maker<V4, Public> {
    /// # Errors
    ///
    /// This function will return an error if the provided private key is invalid.
    pub fn new(private_key: &[u8; 64]) -> Result<Self, MakerError> {
        let private_key = ed25519_dalek::SigningKey::from_keypair_bytes(private_key)
            .map_err(|err| MakerError::InvalidKey(err.to_string()))?;
        let public_key = private_key.verifying_key().to_bytes();
        Ok(Self {
            private_key: Key::<64>::from(&private_key.to_keypair_bytes()),
            public_key: Key::<32>::from(&public_key),
            public_key_bytes: public_key,
            version: V4::NAME.to_string(),
            purpose: Public::NAME.to_string(),
            _version: PhantomData,
            _purpose: PhantomData,
        })
    }

    #[must_use]
    pub fn new_keypair() -> ([u8; 64], [u8; 32]) {
        let mut csprng = rand::rngs::OsRng;
        let priv_key: ed25519_dalek::SigningKey = ed25519_dalek::SigningKey::generate(&mut csprng);
        let pub_key = priv_key.verifying_key();
        (priv_key.to_keypair_bytes(), pub_key.to_bytes())
    }

    fn private_key(&self) -> PasetoAsymmetricPrivateKey<pV4, pPublic> {
        PasetoAsymmetricPrivateKey::<pV4, pPublic>::from(&self.private_key)
    }

    fn public_key(&self) -> PasetoAsymmetricPublicKey<pV4, pPublic> {
        PasetoAsymmetricPublicKey::<pV4, pPublic>::from(&self.public_key)
    }

    #[must_use]
    pub const fn public_key_as_bytes(&self) -> &[u8; 32] {
        &self.public_key_bytes
    }

    /// # Errors
    ///
    /// This function will return an error if the token verification fails.
    pub fn verify_token(&self, token: &str) -> Result<Claims, TokenError> {
        let public_key = self.public_key();
        let mut parser = rusty_paseto::prelude::PasetoParser::<pV4, pPublic>::default();

        let token = {
            parser
                .parse(token, &public_key)
                .map_err(|err| TokenError::TokenCreationFailed(err.to_string()))?
        };
        Ok(token.into())
    }

    /// # Errors
    ///
    /// This function will return an error if the token creation fails due to invalid claims or other issues.
    ///
    pub fn create_token(&self, claims: &Claims) -> Result<String, TokenError> {
        let mut builder = PasetoBuilder::<pV4, pPublic>::default();

        for (key, value) in claims.iter() {
            // dbg!(key, format!("{}", value.to_string().trim_matches('"').to_string()));
            match key.as_ref() {
                reserved::ISSUER => {
                    if let Some(issuer) = value.as_str() {
                        let _ = builder.set_claim(IssuerClaim::from(issuer));
                    } else {
                        return Err(TokenError::InvalidClaim("Invalid issuer claim".to_string()));
                    }
                }
                reserved::AUDIENCE => {
                    if let Some(audience) = value.as_str() {
                        let _ = builder.set_claim(AudienceClaim::from(audience));
                    } else {
                        return Err(TokenError::InvalidClaim(
                            "Invalid audience claim".to_string(),
                        ));
                    }
                }
                reserved::SUBJECT => {
                    if let Some(subject) = value.as_str() {
                        let _ = builder.set_claim(SubjectClaim::from(subject));
                    } else {
                        return Err(TokenError::InvalidClaim(
                            "Invalid subject claim".to_string(),
                        ));
                    }
                }
                reserved::ISSUED_AT => {
                    if let Some(issued_at) = value.as_str() {
                        match IssuedAtClaim::try_from(issued_at) {
                            Ok(claim) => {
                                let _ = builder.set_claim(claim);
                            }
                            Err(err) => {
                                return Err(TokenError::ClaimError(err.into()));
                            }
                        }
                    } else {
                        return Err(TokenError::InvalidClaim(
                            "Invalid issued at claim".to_string(),
                        ));
                    }
                }
                reserved::NOT_BEFORE => {
                    if let Some(not_before) = value.as_str() {
                        match NotBeforeClaim::try_from(not_before) {
                            Ok(claim) => {
                                let _ = builder.set_claim(claim);
                            }
                            Err(err) => {
                                return Err(TokenError::ClaimError(err.into()));
                            }
                        }
                    } else {
                        return Err(TokenError::InvalidClaim(
                            "Invalid not before claim".to_string(),
                        ));
                    }
                }
                reserved::EXPIRATION => {
                    if let Some(expiration) = value.as_str() {
                        match ExpirationClaim::try_from(expiration) {
                            Ok(claim) => {
                                let _ = builder.set_claim(claim);
                            }
                            Err(err) => {
                                return Err(TokenError::ClaimError(err.into()));
                            }
                        }
                    } else {
                        return Err(TokenError::InvalidClaim(
                            "Invalid expiration claim".to_string(),
                        ));
                    }
                }
                reserved::TOKEN_IDENTIFIER => {
                    let claim = match value.as_str() {
                        Some(token_id) => TokenIdentifierClaim::from(token_id),
                        None => {
                            return Err(TokenError::InvalidClaim(
                                "Invalid token identifier claim".to_string(),
                            ))
                        }
                    };
                    let _ = builder.set_claim(claim);
                }
                key => match CustomClaim::try_from((key, value)) {
                    Ok(claim) => {
                        let _ = builder.set_claim(claim);
                    }
                    Err(err) => {
                        return Err(TokenError::InvalidClaim(err.to_string()));
                    }
                },
            }
        }

        builder
            .build(&self.private_key())
            .map_err(|err| TokenError::TokenCreationFailed(err.to_string()))
    }
}

#[cfg(test)]
mod test {

    use std::{
        fs::File,
        io::{Read, Write},
    };

    use rusty_paseto::prelude::PasetoParser;

    use super::*;

    #[test]
    fn test_invalid_claims() {
        let (priv_key, _) = Maker::new_keypair();
        let maker = Maker::new(&priv_key).expect("failed to create maker");

        let claims = Claims::new().with_issued_at("invalid RF3339 date");
        let token = maker.create_token(&claims);
        assert!(token.is_err());
    }

    #[test]
    fn test_create_token() {
        let (priv_key, _) = Maker::new_keypair();
        let maker = Maker::new(&priv_key).expect("failed to create maker");

        let public_key = maker.public_key();
        let mut claims = Claims::new().with_issued_at("2027-09-18T03:42:15+02:00");
        claims.set_claim("sub", "this is the subject").unwrap();
        claims.set_claim("data", "test").unwrap();
        claims.set_claim("number", 2).unwrap();

        let token = maker
            .create_token(&claims)
            .expect("failed to generate token");

        let got = maker.verify_token(&token).expect("failed to verify token");

        assert_eq!(got.get_subject().unwrap().as_str(), "this is the subject");
        let mut parser = PasetoParser::<pV4, pPublic>::default();
        let token = parser
            .parse(&token, &public_key)
            .expect("failed to parse token");
        dbg!(&token);

        assert!(token.get("sub").is_some());
        assert_eq!(
            token.get("sub").unwrap().as_str().unwrap(),
            "this is the subject"
        );
        assert!(token.get("number").is_some());
        assert_eq!(token.get("number").unwrap(), 2);
        assert!(token.get("data").is_some());
        assert_eq!(token.get("data").unwrap(), "test");
    }

    #[test]
    fn test_new_private_key() {
        let new_key = Key::<64>::try_new_random().unwrap();
        // let private_key = PasetoAsymmetricPrivateKey::<V4, Public>::from(&new_key);
        let mut file = File::create("temp_dev_private_key").unwrap();
        file.write_all(new_key.as_slice()).unwrap();

        let mut file = File::open("temp_dev_private_key").unwrap();
        let mut private_key = Vec::new();
        file.read_to_end(&mut private_key).unwrap();

        let got_key = Key::<64>::from(private_key.as_slice());

        assert_eq!(got_key.as_slice(), new_key.as_slice());

        std::fs::remove_file("temp_dev_private_key").unwrap();
    }
}
