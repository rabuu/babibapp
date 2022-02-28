use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use babibapp_schema::schema::student_comment_votes;
use babibapp_schema::schema::student_comments;
use babibapp_schema::schema::teacher_comment_votes;
use babibapp_schema::schema::teacher_comments;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct StudentComment {
    pub id: i32,
    pub author_id: i32,
    pub receiver_id: i32,
    pub body: String,
    pub published: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "student_comments"]
pub struct NewStudentComment {
    pub author_id: i32,
    pub receiver_id: i32,
    pub body: String,
    pub published: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct StudentCommentVote {
    pub id: i32,
    pub comment_id: i32,
    pub student_id: i32,
    pub upvote: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "student_comment_votes"]
pub struct NewStudentCommentVote {
    pub comment_id: i32,
    pub student_id: i32,
    pub upvote: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct TeacherComment {
    pub id: i32,
    pub author_id: i32,
    pub receiver_id: i32,
    pub body: String,
    pub published: SystemTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "teacher_comments"]
pub struct NewTeacherComment {
    pub author_id: i32,
    pub receiver_id: i32,
    pub body: String,
    pub published: Option<SystemTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable)]
pub struct TeacherCommentVote {
    pub id: i32,
    pub comment_id: i32,
    pub student_id: i32,
    pub upvote: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable)]
#[table_name = "teacher_comment_votes"]
pub struct NewTeacherCommentVote {
    pub comment_id: i32,
    pub student_id: i32,
    pub upvote: bool,
}
