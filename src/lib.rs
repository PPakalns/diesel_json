//! Implements utility type for JSON, JSONB field handling in diesel

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;

use diesel::pg::Pg;
use diesel::sql_types;
use diesel::{deserialize::FromSql, serialize::ToSql};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::ops::{Deref, DerefMut};

/// Wrapper type that implements Json data handling for postgres diesel connection
///
/// Use as wrapper for fields that are stored in Jsonb format
/// ```ignore
/// #[derive(serde::Serialize, serde::Deserialize, Debug)]
/// pub struct ComplexStruct {
///   // ...
/// }
///
/// #[derive(serde::Serialize, serde::Deserialize,
///          diesel::Queryable, diesel::Insertable)]
/// pub struct ExampleTable {
///     // Field that will be stored in Json, Jsonb format
///     pub jsonb_field: diesel_json::Json<ComplexStruct>,
/// }
/// ```
#[derive(FromSqlRow, AsExpression, Serialize, Deserialize, Debug, Clone)]
#[serde(transparent)]
#[sql_type = "sql_types::Jsonb"]
pub struct Json<T: Sized>(pub T);

impl<T> Json<T> {
    pub fn new(value: T) -> Json<T> {
        Json(value)
    }
}

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> AsRef<T> for Json<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T> AsMut<T> for Json<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> FromSql<sql_types::Jsonb, Pg> for Json<T>
where
    T: std::fmt::Debug + DeserializeOwned,
{
    fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<sql_types::Jsonb, Pg>>::from_sql(bytes)?;
        Ok(Json(serde_json::from_value::<T>(value)?))
    }
}

impl<T> ToSql<sql_types::Jsonb, Pg> for Json<T>
where
    T: std::fmt::Debug + Serialize,
{
    fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, Pg>,
    ) -> diesel::serialize::Result {
        let value = serde_json::to_value(self)?;
        <serde_json::Value as ToSql<sql_types::Jsonb, Pg>>::to_sql(&value, out)
    }
}

impl<T> PartialEq for Json<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
