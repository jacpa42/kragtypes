use fast_chemail::{parse_email, ParseError};
use serde::{de::Visitor, Deserialize, Serialize};
use smol_str::SmolStr;
use sqlx::{sqlite::SqliteTypeInfo, Decode, Encode, Sqlite, Type};
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

impl<'q> Encode<'q, Sqlite> for EmailAddr {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        Encode::<'q, Sqlite>::encode_by_ref(&self.0.to_string(), buf)
    }
}

impl<'r> Decode<'r, Sqlite> for EmailAddr {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let email = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self(email.into()))
    }
}

impl Type<Sqlite> for EmailAddr {
    fn type_info() -> SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
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
