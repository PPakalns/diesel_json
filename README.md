# diesel_json

Provides a wrapper `diesel_json::Json` type that can be directly used
to wrap serde serializable, deserializable structures and recognize them
as queryable, insertable JsonB fields.

## Getting started

Wrap data structures into `diesel_json::Json` type.

```rust
#[derive(Serialize, Deserialize)]
struct ComplexStruct {
  // ...
}

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset, Identifiable)]
struct ExampleTable {
    // ...
    // Field that will be stored in Jsonb format
    jsonb_field: diesel_json::Json<ComplexStruct>,
    // ...
}
```

`Json` type provides `new` function for object initialization, implements `Deref`,
`DerefMut`, `AsRef`, `AsMut` that maps data access directly to underlying data.

## Why should I use this library?

Without wrapper type for each unique type you store as JsonB field you
would need to use `serde_json::Value` directly
or implement your own implementation for following traits:
```rust
impl<T> FromSql<sql_types::JsonB, Pg> for Json<T> {}
impl<T> ToSql<sql_types::JsonB, Pg> for Json<T> {}
```

## TODO:

- [ ] Support not only JsonB, but also Json postgres type
- [ ] Support more database drivers
    - [x] PostgreSQL
    - [ ] MySql
    - [ ] SQLite
- [ ] Improve testing of library
