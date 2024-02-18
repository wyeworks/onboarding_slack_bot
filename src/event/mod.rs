#[cfg(test)]
mod tests;

use self::{
    challenge::handle_challenge,
    team_join::handle_team_join,
    types::{Event, SlackCallback},
};
use crate::authenticate::authenticate;
use rocket::{http::Status, response::status, serde::json::Json};
mod challenge;
mod team_join;
pub mod types;

#[post("/event", data = "<json_callback>", format = "json")]
pub fn event_route(json_callback: Json<SlackCallback>) -> status::Custom<Json<SlackCallback>> {
    println!("{:?}", json_callback);

    let cb = json_callback.clone().into_inner();

    match cb {
        SlackCallback::Challenge { .. } => handle_challenge(),
        SlackCallback::EventCallback {
            event,
            token,
            api_app_id,
        } => {
            if authenticate(&token, &api_app_id).is_ok() {
                handle_event(event)
            } else {
                println!("Authentication failed");
            }
        }
    }

    status::Custom(Status::Ok, json_callback)
}

fn handle_event(event: Event) {
    match event {
        Event::TeamJoin { user } => handle_team_join(&user),
    }
}
