use error::BabibappApiError;
use reqwest::Client as HttpClient;

use crate::types::*;

pub mod error;
pub mod types;

pub struct BabibappClient {
    base_url: String,
    http: HttpClient,
    token: String,
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

        Ok(BabibappClient {
            base_url,
            http,
            token,
        })
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
}
