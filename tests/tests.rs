#[macro_use]
extern crate diesel;

pub mod setup;

use diesel::prelude::*;
use diesel::sql_query;

/// Simple test to see if diesel postgresql connection works
#[test]
fn test_diesel_connection() {
    setup::setup();

    #[derive(QueryableByName, Debug, PartialEq)]
    struct SimpleQuery {
        #[sql_type = "diesel::sql_types::Integer"]
        number: i32,
    }

    let connection = setup::transaction_connection();
    let result: Vec<SimpleQuery> = sql_query("SELECT 1 as number").load(&connection).unwrap();
    assert_eq!(result, vec![SimpleQuery { number: 1 }]);
}
