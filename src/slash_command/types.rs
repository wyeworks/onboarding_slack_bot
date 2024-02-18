use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(tag = "command")]
pub enum SlackCommand {
    ListNewPeople,
}

#[derive(FromForm, Debug)]
pub struct ListNewsEmployeesCommand {
    pub token: String,
    pub api_app_id: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
}
