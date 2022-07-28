use config::{Config, ConfigError, File};
use serde::Deserialize;

const DEVELOPMENT: &str = "Development";
const PRODUCTION: &str = "Production";

#[derive(Debug, Deserialize, Clone)]
pub struct APIEnvironmentVariables {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvironmentVariables {
    pub api_env: APIEnvironmentVariables,
}

pub fn get_environment_variables() -> Result<EnvironmentVariables, ConfigError> {
    let cur_env = get_current_running_env();
    let cur_config_path = get_config_path(&cur_env);

    println!("Currently running in enviroment => {}", cur_env);
    let env_vars = Config::builder()
        .add_source(File::with_name(&cur_config_path))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        // .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    return env_vars.try_deserialize::<EnvironmentVariables>();
}

fn get_current_running_env() -> String {
    std::env::var("RUN_ENV").unwrap_or_else(|_| DEVELOPMENT.into())
}

fn get_config_path(cur_env: &str) -> String {
    let path = match cur_env {
        DEVELOPMENT => "../configs/development",
        PRODUCTION => "../configs/production",
        _ => "../configs/development",
    };
    return path.to_string();
}
