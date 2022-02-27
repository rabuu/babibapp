table! {
    students (id) {
        id -> Int4,
        email -> Text,
        password_hash -> Text,
        first_name -> Varchar,
        last_name -> Varchar,
        is_admin -> Bool,
    }
}
