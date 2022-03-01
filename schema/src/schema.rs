table! {
    student_comments (id) {
        id -> Int4,
        author_id -> Int4,
        receiver_id -> Int4,
        body -> Text,
        published -> Timestamp,
    }
}

table! {
    student_comment_votes (id) {
        id -> Int4,
        comment_id -> Int4,
        student_id -> Int4,
        upvote -> Bool,
    }
}

table! {
    students (id) {
        id -> Int4,
        email -> Text,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Text,
        admin -> Bool,
    }
}

table! {
    teacher_comments (id) {
        id -> Int4,
        author_id -> Int4,
        receiver_id -> Int4,
        body -> Text,
        published -> Timestamp,
    }
}

table! {
    teacher_comment_votes (id) {
        id -> Int4,
        comment_id -> Int4,
        student_id -> Int4,
        upvote -> Bool,
    }
}

table! {
    teachers (id) {
        id -> Int4,
        name -> Varchar,
        prefix -> Varchar,
    }
}

joinable!(student_comment_votes -> student_comments (comment_id));
joinable!(student_comment_votes -> students (student_id));
joinable!(teacher_comment_votes -> students (student_id));
joinable!(teacher_comment_votes -> teacher_comments (comment_id));
joinable!(teacher_comments -> students (author_id));
joinable!(teacher_comments -> teachers (receiver_id));

allow_tables_to_appear_in_same_query!(
    student_comments,
    student_comment_votes,
    students,
    teacher_comments,
    teacher_comment_votes,
    teachers,
);
