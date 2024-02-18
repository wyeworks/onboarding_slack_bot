use dotenv::dotenv;
use std::env;

const ENV_VAR_NAMES: [&str; 7] = [
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

    #[test]
    fn init() {
        assert_eq!(1, 1)
    }
}
