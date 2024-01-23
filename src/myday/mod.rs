mod session;
mod sessions;

pub use self::sessions::Session;
use self::sessions::{RegisterSessionRequest, RegisterSessionResponse, SessionResponse};

pub struct Client {
    client: reqwest::Client,
    token: String,
    device_code: String,
}

impl Client {
    pub fn new(token: String, device_code: String) -> Self {
        Client {
            client: reqwest::Client::new(),
            token,
            device_code,
        }
    }

    fn session_token(&self) -> String {
        let jwt: jwt::Token<jwt::Header, session::TokenClaims, _> =
            jwt::Token::parse_unverified(&self.token).expect("token should be valid");
        jwt.claims().sid.clone()
    }

    async fn get_access_token(&self) -> Result<String, ()> {
        Ok(self
            .client
            .post("https://api.myday.cloud/sessions/token")
            .form(&[
                ("grant_type", "get_token"),
                (
                    "client_id",
                    "myday-mobile-af53151c-8124-4f7b-9979-7169fcf64bf1",
                ),
                ("id_token", &self.token),
                ("code", &self.device_code),
            ])
            .send()
            .await
            .unwrap()
            .json::<session::TokenResponse>()
            .await
            .unwrap()
            .access_token)
    }

    pub async fn get_expiry(&self) -> Result<String, ()> {
        let access_token = self.get_access_token().await?;
        Ok(self
            .client
            .get(format!(
                "https://api.myday.cloud/sessions/session/{}",
                self.session_token()
            ))
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .unwrap()
            .json::<session::SessionResponse>()
            .await
            .unwrap()
            .expires)
    }

    pub async fn get_sessions_from_date(
        &self,
        start_time: &str,
        end_time: &str,
    ) -> Result<Vec<Session>, ()> {
        let access_token = self.get_access_token().await.unwrap();
        Ok(self
            .client
            .get("https://api.myday.cloud/legacy/api/endpoint/CISConnectLite/sessions")
            .query(&[("endDateTime", end_time), ("startDateTime", start_time)])
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .unwrap()
            .json::<Vec<SessionResponse>>()
            .await
            .unwrap()
            .into_iter()
            .map(Session::from)
            .collect())
    }

    pub async fn get_sessions_from_code(&self, registration_code: u64) -> Result<Vec<Session>, ()> {
        let access_token = self.get_access_token().await.unwrap();
        Ok(self
            .client
            .get("https://api.myday.cloud/legacy/api/endpoint/CISConnectLite/search")
            .query(&[("RegistrationCode", registration_code)])
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .unwrap()
            .json::<Vec<SessionResponse>>()
            .await
            .unwrap()
            .into_iter()
            .map(Session::from)
            .collect())
    }

    pub async fn register_session(
        &self,
        session_id: u64,
        registration_code: String,
    ) -> Result<(), RegisterError> {
        let access_token = self.get_access_token().await.unwrap();
        let response = self
            .client
            .post("https://api.myday.cloud/legacy/api/endpoint/CISConnectLite/register")
            .json(&RegisterSessionRequest {
                session_id,
                registration_code,
                force_incorrect_session_registration: false,
            })
            .header("Authorization", format!("Bearer {}", access_token))
            .send()
            .await
            .unwrap()
            .json::<RegisterSessionResponse>()
            .await
            .unwrap();
        if response.status != "200" || !response.ok {
            return Err(RegisterError::InvalidSessionDetails);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RegisterError {
    InvalidSessionDetails,
    RequestError,
}
