use dotenv::dotenv;
use std::env;

const ENV_VAR_NAMES: [&str; 8] = [
    "APP_ADDRESS",
    "APP_PORT",
    "APP_BASE_ROUTE",
    "SLACK_TOKEN",
    "SLACK_API_APP_ID",
    "REDIS_HOSTNAME",
    "REDIS_PASSWORD",
    "REDIS_URI_SCHEME",
];

pub fn load_env() {
    fn validate_env_vars() {
        for env_var in ENV_VAR_NAMES.iter() {
            match env::var(env_var) {
                Ok(_) => {}
                Err(_) => panic!("{} is not set", env_var),
            }
        }
    }

    match env::var("APP_ENV") {
        Ok(_) => {
            println!("Using environment variables");
        }
        Err(_) => {
            dotenv().ok();
            println!("Using .env file");
        }
    }
    validate_env_vars();
}

#[cfg(test)]
mod tests_load_env {
    use super::load_env;

    fn clear_env() {
        for env_var in super::ENV_VAR_NAMES.iter() {
            std::env::remove_var(env_var);
        }
    }

    fn preload_mocked_vars(iterarable: &[&str]) {
        for env_var in iterarable.iter() {
            std::env::set_var(env_var, "abc");
        }
    }

    #[test]
    fn should_pass_if_all_vars_are_setted() {
        clear_env();

        preload_mocked_vars(&super::ENV_VAR_NAMES);
        load_env()
    }

    #[test]
    #[should_panic]
    fn should_panic_if_any_var_is_not_setted() {
        clear_env();

        preload_mocked_vars(&["APP_ENV", "APP_PORT"]);
        load_env()
    }
}
