/*
use diesel::prelude::PgConnection;
use diesel::{QueryResult, RunQueryDsl};
use crate::db::models::MyData;

pub fn get_all_data(conn: &mut PgConnection) -> QueryResult<Vec<MyData>> {
    use crate::db::schema::my_data::dsl::*;

    my_data.load::<MyData>(conn)
}
*/