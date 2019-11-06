use env_logger;
use rand::Rng;
use structopt;

use code_review_bot::{db, start_dev_server, start_server, AppConfig, AppData};
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

    // Load variables and language lookup
    let github_client_id =
        std::env::var("GITHUB_CLIENT_ID").expect("Can't find var GITHUB_CLIENT_ID");
    let github_client_secret =
        std::env::var("GITHUB_CLIENT_SECRET").expect("Can't find var GITHUB_CLIENT_SECRET");
    let slack_token = std::env::var("SLACK_TOKEN").expect("Can't find var SLACK_TOKEN");
    let slack_channel = std::env::var("SLACK_CHANNEL").expect("Can't find var SLACK_CHANNEL");
    let slack_client_id = std::env::var("SLACK_CLIENT_ID").expect("Can't find var SLACK_CLIENT_ID");
    let slack_client_secret =
        std::env::var("SLACK_CLIENT_SECRET").expect("Can't find var SLACK_CLIENT_SECRET");
    let database_url = std::env::var("DATABASE_URL").expect("Can't find var DATABASE_URL");
    let app_url = std::env::var("APP_URL").expect("Can't find var APP_URL");

    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| {
        String::from_utf8_lossy(&rand::thread_rng().gen::<[u8; 32]>()).into_owned()
    });

    // Setup database
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Can't create conneciton pool");
    let db = db::DBExecutor(pool.clone());

    let builder = AppData::new(db, app_url)
        .github(&github_client_id, &github_client_secret)
        .slack(
            &slack_client_id,
            &slack_client_secret,
            &slack_channel,
            &slack_token,
        );
    // Create AppConfig
    let app_config = AppConfig::new(builder.clone(), builder.build());

    if opt.dev {
        start_dev_server(opt.port, app_config, app_secret)
    } else {
        start_server(opt.port, app_config, app_secret)
    }
    .expect("Could not start server");
}
