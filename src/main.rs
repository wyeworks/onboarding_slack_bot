mod authenticate;
mod database;
mod event;
mod slash_command;
mod utils;

use event::event_route;
use rocket::{Build, Config, Rocket};
use slash_command::help_command_route;
use slash_command::slash_command_route;
use std::env;
use utils::load_env::load_env;
use utils::load_members_from_json::load_members_from_json;
use utils::date_to_timestamp::date_to_timestamp;

use database::get_conn;
use crate::database::{DatabaseActions, Database};

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

    let members = load_members_from_json().expect("Failed to load members from db_seed.json");

    let mut database: Database = get_conn();

    for member in members {
        match date_to_timestamp(&member.date) {
            Ok(ts) => {
                if let Err(e) = database.save_member(&member) {
                    eprintln!("Failed to save member details to database: {}", e);
                }
                if let Err(e) = database.add_member_to_set(&member.id, ts) {
                    eprintln!("Failed to add member to sorted set: {}", e);
                }
            },
            Err(e) => eprintln!("Failed to convert date to timestamp for member {}: {}", member.id, e),
        }
    }

    init_rocket()

}
