use env_logger;
use structopt;

use code_review_bot::{load_languages, start_dev_server, start_server, AppConfig};
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
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let github_token = std::env::var("GITHUB_TOKEN").expect("Can't find var GITHUB_TOKEN");
    let slack_token = std::env::var("SLACK_TOKEN").expect("Can't find var SLACK_TOKEN");
    let language_lookup = load_languages().expect("Can't load language lookup");

    let database_url = std::env::var("DATABASE_URL").expect("Can't find var DATABASE_URL");
    let connection = PgConnection::establish(&database_url).expect("Can't connect to database");

    let app_config = AppConfig::new(&github_token, &slack_token, language_lookup, connection)
        .expect("Can't create app config");

    let opt = Opt::from_args();
    if opt.dev {
        start_dev_server(opt.port, app_config)
    } else {
        start_server(opt.port, app_config)
    }
    .expect("Could not start server");
}
