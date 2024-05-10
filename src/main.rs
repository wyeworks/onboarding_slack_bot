mod authenticate;
mod database;
mod event;
mod pg_database;
mod slash_command;
mod utils;

use event::event_route;
use rocket::{Build, Config, Rocket};
use slash_command::help_command_route;
use slash_command::slash_command_route;
use std::{env, net::IpAddr};
use utils::load_env::load_env;

#[macro_use]
extern crate rocket;
extern crate dotenv;
extern crate redis;

fn init_rocket() -> Rocket<Build> {
    let _pg_connection = pg_database::establish_connection();

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
        routes![event_route, slash_command_route, help_command_route],
    )
}

#[launch]
fn init() -> _ {
    load_env();

    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        let command = &args[1];
        let file_path = &args[2];
        if command == "seed-db" {
            match database::db_seeder::seed_database(file_path) {
                Ok(_) => {
                    println!("Database seeded successfully.");
                }
                Err(e) => {
                    eprintln!("Failed to seed database: {}", e);
                }
            }
        }
    }

    init_rocket()
}
