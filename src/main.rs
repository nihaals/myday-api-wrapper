mod myday;

use std::env;

use rocket::serde::json::Json;
use rocket::{get, routes};
use serde::Serialize;

struct State {
    client: myday::Client,
}

#[derive(Serialize)]
struct ExpiryResponse {
    expiry: String,
}

#[get("/expiry")]
async fn expiry(state: &rocket::State<State>) -> Json<ExpiryResponse> {
    Json(ExpiryResponse {
        expiry: state.client.get_expiry().await.unwrap(),
    })
}

#[get("/sessions?<start_time>&<end_time>&<registration_code>")]
async fn sessions(
    state: &rocket::State<State>,
    start_time: Option<String>,
    end_time: Option<String>,
    registration_code: Option<u64>,
) -> Json<Vec<myday::Session>> {
    if let Some(registration_code) = registration_code {
        return Json(
            state
                .client
                .get_sessions_from_code(registration_code)
                .await
                .unwrap(),
        );
    }
    if let (Some(start_time), Some(end_time)) = (start_time, end_time) {
        return Json(
            state
                .client
                .get_sessions_from_date(&start_time, &end_time)
                .await
                .unwrap(),
        );
    }
    panic!()
}

#[rocket::launch]
fn rocket() -> _ {
    env_logger::init();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_owned());
    let port: u16 = env::var("PORT")
        .unwrap_or("8000".to_owned())
        .parse()
        .unwrap();
    let figment = rocket::Config::figment()
        .merge(("address", &host))
        .merge(("port", port));
    println!("Listening on http://{host}:{port}...");
    rocket::custom(figment)
        .manage(State {
            client: myday::Client::new(
                env::var("MAW_TOKEN").expect("MAW_TOKEN should be set"),
                env::var("MAW_DEVICE_CODE").expect("MAW_DEVICE_CODE should be set"),
            ),
        })
        .mount("/", routes![expiry, sessions])
}
