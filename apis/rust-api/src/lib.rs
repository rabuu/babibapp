use error::BabibappApiError;
use reqwest::Client as HttpClient;

use crate::types::*;
use babibapp_models::wrappers::{EmailWrapper, NameWrapper, PasswordWrapper, TokenWrapper};

pub mod error;
pub mod types;

pub struct BabibappClient {
    base_url: String,
    http: HttpClient,
    token: String,
    pub id: i32,
}

impl BabibappClient {
    pub async fn login(
        base_url: &str,
        email: &str,
        password: &str,
    ) -> Result<BabibappClient, BabibappApiError> {
        let http = HttpClient::new();

        let login = LoginStudent {
            email: email.to_string(),
            password: password.to_string(),
        };

        let TokenWrapper { token } = http
            .post(format!("{}/token/generate", base_url))
            .json(&login)
            .send()
            .await?
            .json()
            .await?;

        let me: Student = http
            .get(format!("{}/student/get_self", base_url))
            .bearer_auth(&token)
            .send()
            .await?
            .json()
            .await?;

        Ok(BabibappClient {
            base_url: base_url.to_string(),
            http,
            token,
            id: me.id,
        })
    }

    pub async fn validate_token(&self) -> Result<bool, BabibappApiError> {
        let token = TokenWrapper {
            token: self.token.clone(),
        };

        let response = self
            .http
            .post(format!("{}/token/validate", self.base_url))
            .json(&token)
            .send()
            .await?
            .text()
            .await?;

        Ok(response == "Valid token")
    }

    pub async fn get_student(&self, student_id: i32) -> Result<StudentView, BabibappApiError> {
        let student = self
            .http
            .get(format!("{}/student/get/{}", self.base_url, student_id))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(student)
    }

    pub async fn get_self(&self) -> Result<Student, BabibappApiError> {
        let student = self
            .http
            .get(format!("{}/student/get_self", self.base_url))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(student)
    }

    pub async fn get_all_students(&self) -> Result<Vec<StudentView>, BabibappApiError> {
        let students = self
            .http
            .get(format!("{}/student/get_all", self.base_url))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(students)
    }

    pub async fn register_student(
        &self,
        email: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
        admin: Option<bool>,
    ) -> Result<Student, BabibappApiError> {
        let new_student = RegisterStudent {
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password: password.to_string(),
            admin,
        };

        let student = self
            .http
            .post(format!("{}/student/register", self.base_url))
            .json(&new_student)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(student)
    }

    pub async fn reset_student_email(
        &self,
        student_id: i32,
        email: &str,
    ) -> Result<Student, BabibappApiError> {
        let email = EmailWrapper {
            email: email.to_string(),
        };

        let student = self
            .http
            .put(format!(
                "{}/student/reset_email/{}",
                self.base_url, student_id
            ))
            .json(&email)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(student)
    }

    pub async fn reset_student_password(
        &self,
        student_id: i32,
        password: &str,
    ) -> Result<Student, BabibappApiError> {
        let password = PasswordWrapper {
            password: password.to_string(),
        };

        let student = self
            .http
            .put(format!(
                "{}/student/reset_password/{}",
                self.base_url, student_id
            ))
            .json(&password)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(student)
    }

    pub async fn reset_student_name(
        &self,
        student_id: i32,
        first_name: &str,
        last_name: &str,
    ) -> Result<Student, BabibappApiError> {
        let name = NameWrapper {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        };

        let student = self
            .http
            .put(format!(
                "{}/student/reset_name/{}",
                self.base_url, student_id
            ))
            .json(&name)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(student)
    }

    pub async fn make_student_admin(&self, student_id: i32) -> Result<Student, BabibappApiError> {
        let student = self
            .http
            .put(format!(
                "{}/student/make_admin/{}",
                self.base_url, student_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(student)
    }

    pub async fn reset_student_full(
        &self,
        student_id: i32,
        email: &str,
        first_name: &str,
        last_name: &str,
        password: &str,
        admin: Option<bool>,
    ) -> Result<Student, BabibappApiError> {
        let reset_student = RegisterStudent {
            email: email.to_string(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            password: password.to_string(),
            admin,
        };

        let student = self
            .http
            .put(format!(
                "{}/student/reset_full/{}",
                self.base_url, student_id
            ))
            .json(&reset_student)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(student)
    }

    pub async fn delete_student(&self, student_id: i32) -> Result<Student, BabibappApiError> {
        let student = self
            .http
            .delete(format!("{}/student/delete/{}", self.base_url, student_id))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(student)
    }

    pub async fn get_teacher(&self, teacher_id: i32) -> Result<Teacher, BabibappApiError> {
        let teacher = self
            .http
            .get(format!("{}/teacher/get/{}", self.base_url, teacher_id))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(teacher)
    }

    pub async fn get_all_teachers(&self) -> Result<Vec<Teacher>, BabibappApiError> {
        let teachers = self
            .http
            .get(format!("{}/teacher/get_all", self.base_url))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(teachers)
    }

    pub async fn add_teacher(&self, name: &str, prefix: &str) -> Result<Teacher, BabibappApiError> {
        let new_teacher = NewTeacher {
            name: name.to_string(),
            prefix: prefix.to_string(),
        };

        let teacher = self
            .http
            .post(format!("{}/teacher/add", self.base_url))
            .json(&new_teacher)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(teacher)
    }

    pub async fn reset_teacher(
        &self,
        teacher_id: i32,
        name: &str,
        prefix: &str,
    ) -> Result<Teacher, BabibappApiError> {
        let reset_teacher = NewTeacher {
            name: name.to_string(),
            prefix: prefix.to_string(),
        };

        let teacher = self
            .http
            .put(format!("{}/teacher/reset/{}", self.base_url, teacher_id))
            .json(&reset_teacher)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(teacher)
    }

    pub async fn delete_teacher(&self, teacher_id: i32) -> Result<Teacher, BabibappApiError> {
        let teacher = self
            .http
            .delete(format!("{}/teacher/delete/{}", self.base_url, teacher_id))
            .send()
            .await?
            .json()
            .await?;
        Ok(teacher)
    }

    pub async fn get_student_comment(
        &self,
        comment_id: i32,
    ) -> Result<StudentCommentView, BabibappApiError> {
        let comment = self
            .http
            .get(format!(
                "{}/comment/student/get/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(comment)
    }

    pub async fn get_all_student_comments(
        &self,
    ) -> Result<Vec<StudentCommentView>, BabibappApiError> {
        let comments = self
            .http
            .get(format!("{}/comment/student/get_all", self.base_url))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(comments)
    }

    pub async fn get_student_comment_vote(&self, comment_id: i32) -> Result<i64, BabibappApiError> {
        let vote = self
            .http
            .get(format!(
                "{}/comment/student/get_vote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(vote)
    }

    pub async fn create_student_comment(
        &self,
        receiver_id: i32,
        body: &str,
    ) -> Result<StudentComment, BabibappApiError> {
        let new_comment = CreateStudentComment {
            receiver_id,
            body: body.to_string(),
        };

        let comment = self
            .http
            .post(format!("{}/comment/student/create", self.base_url))
            .json(&new_comment)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(comment)
    }

    pub async fn upvote_student_comment(&self, comment_id: i32) -> Result<(), BabibappApiError> {
        self.http
            .post(format!(
                "{}/comment/student/upvote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn downvote_student_comment(&self, comment_id: i32) -> Result<(), BabibappApiError> {
        self.http
            .post(format!(
                "{}/comment/student/downvote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn unvote_student_comment(&self, comment_id: i32) -> Result<(), BabibappApiError> {
        self.http
            .post(format!(
                "{}/comment/student/unvote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_student_comment(
        &self,
        comment_id: i32,
    ) -> Result<StudentComment, BabibappApiError> {
        let comment = self
            .http
            .delete(format!(
                "{}/comment/student/delete/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(comment)
    }

    pub async fn get_teacher_comment(
        &self,
        comment_id: i32,
    ) -> Result<TeacherCommentView, BabibappApiError> {
        let comment = self
            .http
            .get(format!(
                "{}/comment/teacher/get/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(comment)
    }

    pub async fn get_all_teacher_comments(
        &self,
    ) -> Result<Vec<TeacherCommentView>, BabibappApiError> {
        let comments = self
            .http
            .get(format!("{}/comment/teacher/get_all", self.base_url))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(comments)
    }

    pub async fn get_teacher_comment_vote(&self, comment_id: i32) -> Result<i64, BabibappApiError> {
        let vote = self
            .http
            .get(format!(
                "{}/comment/teacher/get_vote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(vote)
    }

    pub async fn create_teacher_comment(
        &self,
        receiver_id: i32,
        body: &str,
    ) -> Result<TeacherComment, BabibappApiError> {
        let new_comment = CreateTeacherComment {
            receiver_id,
            body: body.to_string(),
        };

        let comment = self
            .http
            .post(format!("{}/comment/teacher/create", self.base_url))
            .json(&new_comment)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;

        Ok(comment)
    }

    pub async fn upvote_teacher_comment(&self, comment_id: i32) -> Result<(), BabibappApiError> {
        self.http
            .post(format!(
                "{}/comment/teacher/upvote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn downvote_teacher_comment(&self, comment_id: i32) -> Result<(), BabibappApiError> {
        self.http
            .post(format!(
                "{}/comment/teacher/downvote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn unvote_teacher_comment(&self, comment_id: i32) -> Result<(), BabibappApiError> {
        self.http
            .post(format!(
                "{}/comment/teacher/unvote/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn delete_teacher_comment(
        &self,
        comment_id: i32,
    ) -> Result<TeacherComment, BabibappApiError> {
        let comment = self
            .http
            .delete(format!(
                "{}/comment/teacher/delete/{}",
                self.base_url, comment_id
            ))
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        Ok(comment)
    }
}
