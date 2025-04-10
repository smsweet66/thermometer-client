use std::fs;

use rocket::{
    get,
    http::Status,
    launch, routes,
    serde::{json::Json, Serialize},
};

#[derive(Serialize)]
struct TempuratureResponse {
    tempurature: i32,
    id: String,
    name: String,
    connected: bool,
}

#[get("/tempurature")]
fn get_tempurature() -> Result<Json<TempuratureResponse>, Status> {
    let Ok(tempurature_string) = fs::read_to_string("/dev/thermometer") else {
        return Err(Status::NoContent);
    };

    let Ok(tempurature) = tempurature_string.trim().parse() else {
        return Err(Status::InternalServerError);
    };

    Ok(Json(TempuratureResponse {
        tempurature,
        id: "sensor01".to_owned(),
        name: "office".to_owned(),
        connected: true,
    }))
}

#[launch]
fn tempurature_server() -> _ {
    rocket::build().mount("/", routes![get_tempurature])
}
