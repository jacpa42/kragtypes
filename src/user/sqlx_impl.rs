#![cfg(feature = "sqlite")]
use super::password::PasswordHash;
use super::permissions::Permissions;
use sqlx::{Decode, Encode, Sqlite, Type};

impl Type<Sqlite> for Permissions {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <u32 as sqlx::Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for Permissions {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        Encode::<'q, Sqlite>::encode_by_ref(&self.bits(), buf)
    }
}

impl<'r> Decode<'r, Sqlite> for Permissions {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let val = <u32 as Decode<Sqlite>>::decode(value)?;
        Ok(Self::from_bits_truncate(val))
    }
}

impl<'r> Encode<'r, Sqlite> for PasswordHash {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'r>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as Encode<Sqlite>>::encode_by_ref(&self.data, buf)
    }
}

impl<'r> Decode<'r, Sqlite> for PasswordHash {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let data = <String as Decode<Sqlite>>::decode(value)?;
        Ok(Self { data })
    }
}

impl Type<Sqlite> for PasswordHash {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <str as Type<Sqlite>>::type_info()
    }
}
