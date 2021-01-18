pub mod setup;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde;

pub mod schema {
    table! {
        jsonb_test (id) {
            id -> Int4,
            nullable -> Nullable<Jsonb>,
            not_nullable -> Jsonb,
        }
    }
}

use std::collections::HashMap;

use diesel::prelude::*;
use diesel_json::Json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ComplexData {
    pos: usize,
    b: HashMap<String, String>,
}

#[derive(Queryable)]
struct JsonbTest {
    _id: i32,
    nullable: Option<Json<ComplexData>>,
    not_nullable: Json<ComplexData>,
}

use schema::jsonb_test;

#[derive(Insertable)]
#[table_name = "jsonb_test"]
struct JsonbTestInsert {
    nullable: Option<Json<ComplexData>>,
    not_nullable: Json<ComplexData>,
}

#[test]
fn test_insert_and_select() {
    setup::setup();

    let test_data = vec![
        JsonbTestInsert {
            nullable: Some(Json::new(ComplexData {
                pos: 2132,
                b: HashMap::new(),
            })),
            not_nullable: Json::new(ComplexData {
                pos: 0,
                b: HashMap::new(),
            }),
        },
        JsonbTestInsert {
            nullable: None,
            not_nullable: Json::new(ComplexData {
                pos: 1,
                b: HashMap::new(),
            }),
        },
    ];

    let connection = setup::transaction_connection();
    let result = diesel::insert_into(schema::jsonb_test::dsl::jsonb_test)
        .values(&test_data)
        .get_results::<JsonbTest>(&connection)
        .expect("Error inserting and retrieving inserted data");
    assert_eq!(result.len(), 2);

    // Check that nullable and non nullable structures matches
    for res in result.iter() {
        let prev_res = &test_data[res.not_nullable.pos];
        assert_eq!(prev_res.not_nullable, res.not_nullable);
        assert_eq!(prev_res.nullable, res.nullable);
    }
}
