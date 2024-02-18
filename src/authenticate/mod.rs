#[cfg(test)]
mod tests;

use std::env;

pub fn authenticate(token: &str, api_app_id: &str) -> Result<(), ()> {
    let env_slack_token = env::var("SLACK_TOKEN").unwrap();
    let env_slack_api_app_id = env::var("SLACK_API_APP_ID").unwrap();

    if token != env_slack_token || api_app_id != env_slack_api_app_id {
        return Err(());
    }
    Ok(())
}
