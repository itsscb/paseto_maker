use crate::errors::ClaimError;

use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fmt::{self, Display, Formatter},
    sync::Arc,
};

pub mod reserved;

/// Represents a collection of claims for a token.
///
/// Claims are stored in a `BTreeMap` with `Arc<str>` keys and `serde_json::Value` values.
///
/// # Examples
///
/// ```
/// use paseto_maker::Claims;
///
/// let claims = Claims::new()
///     .with_subject("1234567890")
///     .with_issuer("issuer")
///     .with_audience("audience")
///     .with_expiration("2023-10-01T00:00:00+00:00")
///     .with_not_before("2023-09-01T00:00:00+00:00")
///     .with_issued_at("2023-09-01T00:00:00+00:00")
///     .with_token_identifier("token_id");
///
/// let subject: Option<String> = claims.get_subject();
/// assert_eq!(subject, Some("1234567890".to_string()));
/// ```
///
/// # Methods
///
/// - `new`: Creates a new, empty `Claims` instance.
/// - `with_subject`: Adds a subject claim.
/// - `with_issuer`: Adds an issuer claim.
/// - `with_audience`: Adds an audience claim.
/// - `with_expiration`: Adds an expiration claim.
/// - `with_not_before`: Adds a not-before claim.
/// - `with_issued_at`: Adds an issued-at claim.
/// - `with_token_identifier`: Adds a token identifier claim.
/// - `set_claim`: Sets a custom claim with a specified key and value.
/// - `get_claim`: Retrieves a claim by key and attempts to deserialize it into the specified type.
/// - `get_subject`: Retrieves the subject claim.
/// - `get_issuer`: Retrieves the issuer claim.
/// - `get_audience`: Retrieves the audience claim.
/// - `get_expiration`: Retrieves the expiration claim.
/// - `get_not_before`: Retrieves the not-before claim.
/// - `get_issued_at`: Retrieves the issued-at claim.
/// - `get_token_identifier`: Retrieves the token identifier claim.
/// - `iter`: Returns an iterator over the claims.
///
/// # Errors
///
/// - `set_claim` will return an error if the value cannot be serialized or is null.
/// # Examples
///
/// ```
/// use paseto_maker::Claims;
///
/// let mut claims = Claims::new();
/// claims.set_claim("sub", "1234567890").unwrap();
/// claims.set_claim("name", "John Doe").unwrap();
/// claims.set_claim("admin", true).unwrap();
///
/// let sub: Option<String> = claims.get_claim("sub");
/// assert_eq!(sub, Some("1234567890".to_string()));
///
/// let name: Option<String> = claims.get_claim("name");
/// assert_eq!(name, Some("John Doe".to_string()));
///
/// let admin: Option<bool> = claims.get_claim("admin");
/// assert_eq!(admin, Some(true));
/// ```
///
/// ```
/// use paseto_maker::Claims;
/// use chrono::{DateTime, Utc};
///
/// let claims = Claims::new()
///     .with_subject("1234567890")
///     .with_issuer("issuer")
///     .with_audience("audience")
///     .with_expiration("2023-10-01T00:00:00+00:00")
///     .with_not_before("2023-09-01T00:00:00+00:00")
///     .with_issued_at("2023-09-01T00:00:00+00:00")
///     .with_token_identifier("token_id");
///
/// let subject: Option<String> = claims.get_subject();
/// assert_eq!(subject, Some("1234567890".to_string()));
///
/// let issuer: Option<String> = claims.get_issuer();
/// assert_eq!(issuer, Some("issuer".to_string()));
///
/// let audience: Option<String> = claims.get_audience();
/// assert_eq!(audience, Some("audience".to_string()));
///
/// let expiration: Option<DateTime<Utc>> = claims.get_expiration();
/// assert_eq!(expiration.unwrap().to_rfc3339().to_string(), "2023-10-01T00:00:00+00:00".to_string());
///
/// let not_before: Option<DateTime<Utc>> = claims.get_not_before();
/// assert_eq!(not_before.unwrap().to_rfc3339().to_string(), "2023-09-01T00:00:00+00:00".to_string());
///
/// let issued_at: Option<DateTime<Utc>> = claims.get_issued_at();
/// assert_eq!(issued_at.unwrap().to_rfc3339().to_string(), "2023-09-01T00:00:00+00:00".to_string());
///
/// let token_identifier: Option<String> = claims.get_token_identifier();
/// assert_eq!(token_identifier, Some("token_id".to_string()));
/// ```

#[derive(Debug, Default)]
pub struct Claims {
    claims: BTreeMap<Arc<str>, Value>,
}

impl Display for Claims {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let claims: Vec<String> = self
            .claims
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect();
        write!(f, "{}", claims.join(", "))
    }
}

impl From<Value> for Claims {
    fn from(value: Value) -> Self {
        let claims: BTreeMap<Arc<str>, Value> = match value {
            Value::Object(map) => map.into_iter().map(|(k, v)| (Arc::from(k), v)).collect(),
            _ => BTreeMap::new(),
        };
        Self { claims }
    }
}

impl Claims {
    pub fn iter(&self) -> impl Iterator<Item = (&Arc<str>, &Value)> {
        self.claims.iter()
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_subject<T: AsRef<str>>(mut self, subject: T) -> Self {
        self.claims
            .insert(Arc::from(reserved::SUBJECT), subject.as_ref().into());
        self
    }

    #[must_use]
    pub fn with_issuer<T: AsRef<str>>(mut self, issuer: T) -> Self {
        self.claims
            .insert(Arc::from(reserved::ISSUER), issuer.as_ref().into());
        self
    }

    #[must_use]
    pub fn with_audience<T: AsRef<str>>(mut self, audience: T) -> Self {
        self.claims
            .insert(Arc::from(reserved::AUDIENCE), audience.as_ref().into());
        self
    }

    #[must_use]
    pub fn with_expiration<T: AsRef<str>>(mut self, expiration: T) -> Self {
        self.claims
            .insert(Arc::from(reserved::EXPIRATION), expiration.as_ref().into());
        self
    }

    #[must_use]
    pub fn with_not_before<T: AsRef<str>>(mut self, not_before: T) -> Self {
        self.claims
            .insert(Arc::from(reserved::NOT_BEFORE), not_before.as_ref().into());
        self
    }

    #[must_use]
    pub fn with_issued_at<T: AsRef<str>>(mut self, issued_at: T) -> Self {
        self.claims
            .insert(Arc::from(reserved::ISSUED_AT), issued_at.as_ref().into());
        self
    }

    #[must_use]
    pub fn with_token_identifier<T: AsRef<str>>(mut self, token_identifier: T) -> Self {
        self.claims.insert(
            Arc::from(reserved::TOKEN_IDENTIFIER),
            token_identifier.as_ref().into(),
        );
        self
    }

    /// # Errors
    ///
    /// This function will return an error if the value cannot be serialized.
    /// * `get_claim` - Retrieves a claim by key and attempts to deserialize it into the specified type.
    ///
    pub fn set_claim<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), ClaimError> {
        let value = serde_json::to_value(value)?;
        if value.is_null() {
            return Err(ClaimError::InvalidValue);
        }
        self.claims.insert(Arc::from(key), value);
        Ok(())
    }

    #[must_use]
    pub fn get_claim<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.claims
            .get(&Arc::from(key))
            .and_then(|value| serde_json::from_value(value.clone()).ok())
    }

    #[must_use]
    pub fn get_subject(&self) -> Option<String> {
        self.get_claim(reserved::SUBJECT)
    }

    #[must_use]
    pub fn get_issuer(&self) -> Option<String> {
        self.get_claim(reserved::ISSUER)
    }

    #[must_use]
    pub fn get_audience(&self) -> Option<String> {
        self.get_claim(reserved::AUDIENCE)
    }

    #[must_use]
    pub fn get_expiration(&self) -> Option<DateTime<Utc>> {
        self.get_claim(reserved::EXPIRATION)
    }

    #[must_use]
    pub fn get_not_before(&self) -> Option<DateTime<Utc>> {
        self.get_claim(reserved::NOT_BEFORE)
    }

    #[must_use]
    pub fn get_issued_at(&self) -> Option<DateTime<Utc>> {
        self.get_claim(reserved::ISSUED_AT)
    }

    #[must_use]
    pub fn get_token_identifier(&self) -> Option<String> {
        self.get_claim(reserved::TOKEN_IDENTIFIER)
    }
}

#[cfg(test)]
mod test {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_set_and_get_claim() {
        let mut claims = Claims::new();
        claims.set_claim("sub", "1234567890").unwrap();
        claims.set_claim("name", "John Doe").unwrap();
        claims.set_claim("admin", true).unwrap();

        let sub: Option<String> = claims.get_claim("sub");
        assert_eq!(sub, Some("1234567890".to_string()));

        let name: Option<String> = claims.get_claim("name");
        assert_eq!(name, Some("John Doe".to_string()));

        let admin: Option<bool> = claims.get_claim("admin");
        assert_eq!(admin, Some(true));
    }

    #[test]
    fn test_builder() {
        let claims = Claims::new()
            .with_subject("1234567890")
            .with_audience("test audience")
            .with_issued_at("2019-01-01T00:00:00+00:00");

        let sub: Option<String> = claims.get_claim("sub");
        assert_eq!(sub, Some("1234567890".to_string()));
        let sub: Option<String> = claims.get_claim("aud");
        assert_eq!(sub, Some("test audience".to_string()));

        let sub: Option<String> = claims.get_claim("iat");
        assert_eq!(sub, Some("2019-01-01T00:00:00+00:00".to_string()));
    }

    #[test]
    fn test_iter() {
        let mut claims = Claims::new();
        claims.set_claim("sub", "1234567890").unwrap();
        claims.set_claim("name", "John Doe").unwrap();
        claims.set_claim("admin", true).unwrap();

        let mut iter = claims.iter();
        assert_eq!(iter.next(), Some((&Arc::from("admin"), &json!(true))));
        assert_eq!(iter.next(), Some((&Arc::from("name"), &json!("John Doe"))));
        assert_eq!(iter.next(), Some((&Arc::from("sub"), &json!("1234567890"))));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_get_claim_nonexistent() {
        let claims = Claims::new();
        let value: Option<String> = claims.get_claim("nonexistent");
        assert_eq!(value, None);
    }

    #[test]
    fn test_set_claim_error() {
        let mut claims = Claims::new();
        let result = claims.set_claim("invalid", f64::NAN);
        dbg!(&result);
        assert!(result.is_err());
    }
}
