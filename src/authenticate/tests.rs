#[cfg(test)]
mod test_authenticate {
    use std::env;

    use crate::authenticate::authenticate;

    #[test]
    fn should_pass_if_called_with_correct_params() {
        let token = "some_token";
        let api_app_id = "some_app_id";
        env::set_var("SLACK_TOKEN", token);
        env::set_var("SLACK_API_APP_ID", api_app_id);

        authenticate(token, api_app_id).unwrap();
    }

    #[test]
    fn should_fail_if_called_with_incorrect_params() {
        let token = "some_token";
        let api_app_id = "some_app_id";
        env::set_var("SLACK_TOKEN", token);
        env::set_var("SLACK_API_APP_ID", api_app_id);

        let res = authenticate("asdas", "123");
        assert_eq!(res, Err("Invalid token or app_id".to_string()));
    }
}
