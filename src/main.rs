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
        .mount("/", routes![expiry])
}
