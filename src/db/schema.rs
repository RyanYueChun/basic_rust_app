use diesel::table;


table!{
    my_data (id) {
        id -> Int4,
        value -> VarChar,
    }
}
