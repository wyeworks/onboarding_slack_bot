#[cfg(test)]
mod tests;

use self::{challenge::handle_challenge, team_join::handle_team_join};
use crate::authenticate::authenticate;
use rocket::{http::Status, response::status, serde::json::Json};
mod challenge;
mod team_join;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum SlackCallback {
    #[serde(rename = "url_verification")]
    Challenge { token: String, challenge: String },
    #[serde(rename = "event_callback")]
    EventCallback {
        token: String,
        api_app_id: String,
        event: Event,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "team_join")]
    TeamJoin { user: TeamJoinUser },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamJoinUser {
    pub id: String,
    pub profile: TeamJoinUserProfile,
    pub tz: String,
    pub tz_label: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TeamJoinUserProfile {
    pub email: String,
    pub display_name: String,
}

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Employee {
//     pub id: String,
//     pub email: String,
//     pub full_name: String,
//     pub country: String,
//     pub date: String,
// }

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
        Event::TeamJoin { user } => handle_team_join(user),
    }
}
