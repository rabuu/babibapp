table! {
    students (id) {
        id -> Int4,
        email -> Text,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Text,
        is_admin -> Bool,
    }
}
