mod authenticate;
mod database;
mod event;
mod slash_command;
mod utils;

use event::event_route;
use rocket::{Build, Config, Rocket};
use slash_command::slash_command_route;
use std::{env, net::IpAddr};
use utils::load_env::load_env;

#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate redis;

fn init_rocket() -> Rocket<Build> {
    let config = Config {
        port: env::var("APP_PORT")
            .unwrap()
            .parse()
            .expect("APP_PORT is not a valid number"),
        address: env::var("APP_ADDRESS")
            .unwrap_or("127.0.0.1".to_string())
            .parse::<IpAddr>()
            .unwrap(),
        ..Config::debug_default()
    };

    rocket::build().configure(&config).mount(
        env::var("APP_BASE_ROUTE").unwrap(),
        routes![event_route, slash_command_route],
    )
}

#[launch]
fn init() -> _ {
    load_env();
    init_rocket()
}
