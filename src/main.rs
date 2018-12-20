extern crate code_review_bot;
extern crate env_logger;
extern crate structopt;

use code_review_bot::{start_dev_server, start_server};
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
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let opt = Opt::from_args();
    if opt.dev {
        start_dev_server(opt.port)
    } else {
        start_server(opt.port)
    }.expect("Could not start server");
}
