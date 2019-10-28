use env_logger;
use structopt;

use code_review_bot::{db, load_languages, start_dev_server, start_server, AppConfig};
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
    let github_token = std::env::var("GITHUB_TOKEN").expect("Can't find var GITHUB_TOKEN");
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
    let webhook_url = std::env::var("WEBHOOK_URL").expect("Can't find var WEBHOOK_URL");
    let app_secret = std::env::var("APP_SECRET").expect("Can't find var APP_SECRET");
    let language_lookup = load_languages().expect("Can't load language lookup");

    // Setup database
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::new(manager).expect("Can't create conneciton pool");
    let db = db::DBExecutor(pool.clone());

    // Create AppConfig
    let app_config = AppConfig::new(
        &github_token,
        &github_client_id,
        &github_client_secret,
        &slack_token,
        &slack_channel,
        &slack_client_id,
        &slack_client_secret,
        language_lookup,
        db,
        webhook_url,
        app_secret,
    )
    .expect("Can't create app config");

    if opt.dev {
        start_dev_server(opt.port, app_config)
    } else {
        start_server(opt.port, app_config)
    }
    .expect("Could not start server");
}
