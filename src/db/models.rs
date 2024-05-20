use serde::{Serialize, Deserialize};
use diesel::{Queryable};

// add the derive Selectable ?
#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = my_data)]
pub struct MyData {
    pub id: i32,
    pub value: String,
}