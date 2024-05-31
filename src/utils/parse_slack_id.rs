pub fn parse_slack_id(slack_id: &str) -> String {
    slack_id
        .trim_start_matches("<@")
        .trim_end_matches('>')
        .split('|')
        .next()
        .unwrap()
        .to_string()
}
