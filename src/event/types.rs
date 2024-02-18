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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Member {
    pub id: String,
    pub email: String,
    pub full_name: String,
    pub country: String,
    pub _raw: TeamJoinUser,
}
