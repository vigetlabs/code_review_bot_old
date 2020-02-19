use env_logger;
use rand::Rng;
use std::collections::HashMap;
use structopt;

use code_review_bot::{db, start_dev_server, start_server, AppConfig, AppData, Config};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use structopt::StructOpt;

/// A slack bot server
#[derive(StructOpt, Debug)]
#[structopt(name = "code_review_bot")]
struct Opt {
    /// Run server in dev mode with reloading
    #[structopt(short = "d", long = "dev")]
    dev: bool,

    /// Port
    #[structopt(short = "p", long = "port", default_value = "8088")]
    port: u32,

    /// Log Level
    #[structopt(short = "l", long = "log_level", default_value = "info")]
    log_level: String,
}

fn main() {
    // Load Environment
    dotenv().ok();

    // Get options
    let opt = Opt::from_args();

    // Setup logger
    std::env::set_var(
        "RUST_LOG",
        format!(
            "code_review_bot={level},actix_web={level}",
            level = opt.log_level
        ),
    );
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("Can't find var DATABASE_URL");
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| {
        String::from_utf8_lossy(&rand::thread_rng().gen::<[u8; 32]>()).into_owned()
    });

    // Setup database
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Can't create conneciton pool");
    let db = db::DBExecutor(pool);
    let config_rows = Config::all(&db).expect("Can't get configurations");
    let configs: HashMap<&str, &str> = config_rows
        .iter()
        .map(|config| (config.key.as_ref(), config.value.as_ref()))
        .collect();

    let mut builder = AppData::new();

    if configs.contains_key("github_client_id") && configs.contains_key("github_client_secret") {
        builder = builder.github(
            configs.get("github_client_id").unwrap(),
            configs.get("github_client_secret").unwrap(),
        );
    }

    if configs.contains_key("slack_client_id")
        && configs.contains_key("slack_client_secret")
        && configs.contains_key("slack_channel")
        && configs.contains_key("slack_token")
    {
        builder = builder.slack(
            configs.get("slack_client_id").unwrap(),
            configs.get("slack_client_secret").unwrap(),
            configs.get("slack_channel").unwrap(),
            configs.get("slack_token").unwrap(),
        );
    }

    if configs.contains_key("app_url") {
        builder = builder.app_url(configs.get("app_url").unwrap());
    }

    // Create AppConfig
    let app_config = AppConfig::new(builder.clone(), builder.build());

    if opt.dev {
        start_dev_server(opt.port, app_config, app_secret, db)
    } else {
        start_server(opt.port, app_config, app_secret, db)
    }
    .expect("Could not start server");
}
