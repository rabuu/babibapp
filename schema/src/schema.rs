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

table! {
    teachers (id) {
        id -> Int4,
        name -> Varchar,
        prefix -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    students,
    teachers,
);
