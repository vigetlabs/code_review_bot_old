use env_logger;
use structopt;

use code_review_bot::{load_languages, start_dev_server, start_server};
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
}

fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let github_token = std::env::var("GITHUB_TOKEN").expect("Can't find var GITHUB_TOKEN");
    let language_lookup = load_languages().expect("Can't load language lookup");

    let opt = Opt::from_args();
    if opt.dev {
        start_dev_server(opt.port, github_token, language_lookup)
    } else {
        start_server(opt.port, github_token, language_lookup)
    }
    .expect("Could not start server");
}
