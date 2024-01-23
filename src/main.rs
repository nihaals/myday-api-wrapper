mod myday;

use std::env;
use std::net::SocketAddr;

use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

struct State {
    client: myday::Client,
}

#[derive(Serialize)]
struct ExpiryResponse {
    expiry: String,
}

#[get("/expiry")]
async fn expiry(state: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok().json(ExpiryResponse {
        expiry: state.client.get_expiry().await.unwrap(),
    })
}

#[derive(Deserialize)]
struct SessionsQuery {
    start_time: Option<String>,
    end_time: Option<String>,
    registration_code: Option<u64>,
}

impl SessionsQuery {
    fn start_end_times(&self) -> Option<(&str, &str)> {
        if let (Some(start_time), Some(end_time)) = (&self.start_time, &self.end_time) {
            Some((start_time, end_time))
        } else {
            None
        }
    }

    fn registration_code(&self) -> Option<u64> {
        self.registration_code
    }

    fn is_valid(&self) -> bool {
        (self.registration_code.is_some() && self.start_time.is_none() && self.end_time.is_none())
            || (self.registration_code.is_none() && self.start_end_times().is_some())
    }
}

#[get("/sessions")]
async fn sessions(state: web::Data<State>, query: web::Query<SessionsQuery>) -> HttpResponse {
    if !query.is_valid() {
        return HttpResponse::BadRequest().finish();
    }

    if let Some(registration_code) = query.registration_code() {
        return HttpResponse::Ok().json(
            state
                .client
                .get_sessions_from_code(registration_code)
                .await
                .unwrap(),
        );
    }

    if let Some((start_time, end_time)) = query.start_end_times() {
        return HttpResponse::Ok().json(
            state
                .client
                .get_sessions_from_date(start_time, end_time)
                .await
                .unwrap(),
        );
    }

    unreachable!();
}

#[derive(Deserialize)]
struct RegisterBody {
    session_id: u64,
    registration_code: String,
}

#[post("/register")]
async fn register(state: web::Data<State>, body: web::Json<RegisterBody>) -> HttpResponse {
    match state
        .client
        .register_session(body.session_id, body.registration_code.clone())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(myday::RegisterError::InvalidSessionDetails) => HttpResponse::BadRequest().finish(),
        Err(myday::RegisterError::RequestError) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr: SocketAddr = format!("{}:{}", host, port).parse().unwrap();

    let state = web::Data::new(State {
        client: myday::Client::new(
            env::var("MAW_TOKEN").expect("MAW_TOKEN should be set"),
            env::var("MAW_DEVICE_CODE").expect("MAW_DEVICE_CODE should be set"),
        ),
    });

    println!("Listening on http://{}...", addr);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(state.clone())
            .service(expiry)
            .service(sessions)
    })
    .bind(addr)?
    .run()
    .await
}
