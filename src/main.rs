use std::fs::{self, File};

use daemonize::Daemonize;
use rocket::{
    get,
    http::Status,
    launch, routes,
    serde::{json::Json, Serialize},
    Config,
};

#[derive(Serialize)]
struct TemperatureResponse {
    temperature: i32,
    id: String,
    name: String,
    connected: bool,
}

#[get("/temperature")]
fn get_tempurature() -> Result<Json<TemperatureResponse>, Status> {
    let Ok(temperature_string) = fs::read_to_string("/dev/thermometer") else {
        return Err(Status::NoContent);
    };

    let Ok(temperature) = temperature_string.trim().parse() else {
        return Err(Status::InternalServerError);
    };

    Ok(Json(TemperatureResponse {
        temperature,
        id: "sensor01".to_owned(),
        name: "office".to_owned(),
        connected: true,
    }))
}

#[launch]
fn temperature_server() -> _ {
    let stderr = File::create("/tmp/thermometer.err").expect("Could not create stderr");
    let stdout = File::create("/tmp/thermometer.out").expect("Could not create stdout");

    let daemonize = Daemonize::new()
        .pid_file("/tmp/thermometer.pid")
        .stderr(stderr)
        .stdout(stdout);

    match daemonize.start() {
        Ok(_) => println!("Daemon Started"),
        Err(e) => eprintln!("Error: {e}"),
    };

    let config = Config::figment()
        .merge((Config::PORT, 8000))
        .merge((Config::ADDRESS, "0.0.0.0"));

    rocket::custom(config).mount("/", routes![get_tempurature])
}
