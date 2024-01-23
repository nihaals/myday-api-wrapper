use serde::Deserialize;

pub struct Client {
    client: reqwest::Client,
    token: String,
    device_code: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Deserialize)]
struct SessionResponse {
    expires: String,
}

#[derive(Deserialize)]
struct TokenClaims {
    sid: String,
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
        let jwt: jwt::Token<jwt::Header, TokenClaims, _> =
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
            .json::<TokenResponse>()
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
            .json::<SessionResponse>()
            .await
            .unwrap()
            .expires)
    }
}
