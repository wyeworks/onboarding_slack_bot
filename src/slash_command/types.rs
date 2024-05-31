#[derive(FromForm, Debug)]
pub struct SlackCommand {
    pub token: String,
    pub user_id: String,
    pub api_app_id: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
}
