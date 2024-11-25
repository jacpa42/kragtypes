#![cfg(feature = "sqlite")]

use sqlx::{Decode, Encode, Sqlite, Type};

use super::{session::SessionPass, time::TimePass};
impl Type<Sqlite> for TimePass {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <[u8] as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for TimePass {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        assert_eq!(12, std::mem::size_of::<Self>());
        let raw: &[u8; 12] = unsafe { std::mem::transmute(self) };
        Encode::<Sqlite>::encode(raw.as_slice(), buf)
    }
}

impl<'q> Decode<'q, Sqlite> for TimePass {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'q>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes: &[u8] = Decode::<Sqlite>::decode(value)?;
        assert_eq!(bytes.len(), std::mem::size_of::<Self>());
        let tp_pointer: *const TimePass = unsafe { std::mem::transmute(bytes.as_ptr()) };
        Ok(unsafe { *tp_pointer })
    }
}

impl Type<Sqlite> for SessionPass {
    fn type_info() -> <Sqlite as sqlx::Database>::TypeInfo {
        <[u8] as Type<Sqlite>>::type_info()
    }
}

impl<'q> Encode<'q, Sqlite> for SessionPass {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        assert_eq!(16, std::mem::size_of::<Self>());
        let raw: &[u8; 16] = unsafe { std::mem::transmute(self) };
        Encode::<Sqlite>::encode(raw.as_slice(), buf)
    }
}

impl<'q> Decode<'q, Sqlite> for SessionPass {
    fn decode(
        value: <Sqlite as sqlx::Database>::ValueRef<'q>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let bytes: &[u8] = Decode::<Sqlite>::decode(value)?;
        assert_eq!(bytes.len(), std::mem::size_of::<Self>());
        let tp_pointer: *const SessionPass = unsafe { std::mem::transmute(bytes.as_ptr()) };
        Ok(unsafe { *tp_pointer })
    }
}
