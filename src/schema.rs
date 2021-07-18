table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        clearance -> Bool,
        created_at -> Timestamp,
    }
}
