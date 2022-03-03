use error::BabibappApiError;
use reqwest::Client as HttpClient;

use crate::types::*;

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
        let base_url = format!("http://{}", base_url);
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
            base_url,
            http,
            token,
            id: me.id,
        })
    }

    pub async fn get_student(&self, student_id: i32) -> Result<StudentView, BabibappApiError> {
        let student: StudentView = self
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
        let student: Student = self
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
        let students: Vec<StudentView> = self
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
}
