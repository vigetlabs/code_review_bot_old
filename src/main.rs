use env_logger;
use structopt;

use actix::SyncArbiter;
use code_review_bot::{db, load_languages, start_dev_server, start_server, AppConfig};
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use structopt::StructOpt;

use diesel::prelude::*;

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
}

fn main() {
    // Load Environment
    dotenv().ok();

    // Setup logger
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create actix system to run database actors
    let sys = actix::System::new("cr-bot");

    // Load variables and language lookup
    let github_token = std::env::var("GITHUB_TOKEN").expect("Can't find var GITHUB_TOKEN");
    let slack_token = std::env::var("SLACK_TOKEN").expect("Can't find var SLACK_TOKEN");
    let slack_channel = std::env::var("SLACK_CHANNEL").expect("Can't find var SLACK_CHANNEL");
    let language_lookup = load_languages().expect("Can't load language lookup");
    let database_url = std::env::var("DATABASE_URL").expect("Can't find var DATABASE_URL");

    // Setup database
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Can't create conneciton pool");
    let addr = SyncArbiter::start(3, move || db::DBExecutor(pool.clone()));

    // Create AppConfig
    let app_config = AppConfig::new(
        &github_token,
        &slack_token,
        &slack_channel,
        language_lookup,
        addr,
    )
    .expect("Can't create app config");

    // Get options
    let opt = Opt::from_args();
    if opt.dev {
        start_dev_server(opt.port, app_config)
    } else {
        start_server(opt.port, app_config)
    }
    .expect("Could not start server");

    // Run actix system
    let _ = sys.run();
}
