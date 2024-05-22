diesel::table! {
    students (id) {
        id -> Uuid,
        name -> Varchar,
        age -> Integer,
        department -> Varchar,
    }
}
