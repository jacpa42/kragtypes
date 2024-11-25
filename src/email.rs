use fast_chemail::{parse_email, ParseError};
use serde::{de::Visitor, Deserialize, Serialize};
use smol_str::SmolStr;
use std::str::FromStr;

#[derive(Serialize, PartialEq, Debug, Clone)]
pub struct EmailAddr(SmolStr);

impl<'de> Deserialize<'de> for EmailAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(EmailVisitor)
    }
}

struct EmailVisitor;

impl<'de> Visitor<'de> for EmailVisitor {
    type Value = EmailAddr;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("An rfc5321 compliant email address str")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Self::Value::from_str(v).map_err(|e| E::custom(format!("{e}")))
    }
}

impl AsRef<str> for EmailAddr {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(feature = "sqlite")]
impl<'q> sqlx::Encode<'q, sqlx::Sqlite> for EmailAddr {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        sqlx::Encode::<'q, sqlx::Sqlite>::encode_by_ref(&self.0.to_string(), buf)
    }
}

#[cfg(feature = "sqlite")]
impl<'r> sqlx::Decode<'r, sqlx::Sqlite> for EmailAddr {
    fn decode(
        value: <sqlx::Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let email = <String as sqlx::Decode<sqlx::Sqlite>>::decode(value)?;
        Ok(Self(email.into()))
    }
}

#[cfg(feature = "sqlite")]
impl sqlx::Type<sqlx::Sqlite> for EmailAddr {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

impl FromStr for EmailAddr {
    type Err = ParseError;

    fn from_str(data: &str) -> Result<Self, Self::Err> {
        parse_email(data)?;
        Ok(Self(data.into()))
    }
}

#[test]
fn email_parse() {
    assert!("bruh".parse::<EmailAddr>().is_err());
    assert!("default@gmail.com".parse::<EmailAddr>().is_ok());
}
