use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    /// The goal here is to define which actions a user of the database can perform.
    ///
    /// There are 3 types of users:
    ///
    /// *Member*: They are able to update any passes assigned to their account as well as basic
    /// information such as email, username and password.
    ///
    /// *Admin*: They are able to CRUD the user and pass tables. Additionally they can view financial
    /// information (note that this table does not yet exist).
    ///
    /// *Root*: Can do everything
    // TODO: Write unit tests for the permissions system
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Permissions: u32 {
        const ROOT = !0;
        const NONE = 0;

        const PASS_CRUD =
              Permissions::PASS_CREATE.bits()
            | Permissions::PASS_READ.bits()
            | Permissions::PASS_UPDATE.bits()
            | Permissions::PASS_DELETE.bits();

        const USER_CRUD =
              Permissions::USER_CREATE.bits()
            | Permissions::USER_READ.bits()
            | Permissions::USER_UPDATE.bits()
            | Permissions::USER_DELETE.bits();

        const ADMIN = Self::PASS_CRUD.bits() | Self::USER_CRUD.bits();


        /// Can create users.
        const USER_CREATE      = 1 << 0;
        /// Can read any user data.
        const USER_READ        = 1 << 1;
        /// Can update any user data.
        const USER_UPDATE      = 1 << 2;
        /// Can delete any users (except root).
        const USER_DELETE      = 1 << 3;

        /// Can create passes for any user
        const PASS_CREATE      = 1 << 4;
        /// Can read passes for any user
        const PASS_READ        = 1 << 5;
        /// Can update passes for any user
        const PASS_UPDATE      = 1 << 6;
        /// Can delete passes for any user
        const PASS_DELETE      = 1 << 7;
    }
}

impl Serialize for Permissions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        <u32 as Serialize>::serialize(&self.bits(), serializer)
    }
}

impl<'de> Deserialize<'de> for Permissions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        <u32 as Deserialize>::deserialize(deserializer).map(Permissions::from_bits_truncate)
    }
}
