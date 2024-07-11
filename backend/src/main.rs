extern crate env_logger;
extern crate log;

use chrono::Local;
use dotenv::dotenv;

mod api;
mod core;
mod service;

fn print_version() {
    env_logger::init();
    dotenv().ok();

    let version = env!("CARGO_PKG_VERSION");
    let timestamp = Local::now().format("%d/%m/%Y %H:%M");

    log::info!("Application version v.{} from {}", version, timestamp);
}

fn main() {
    print_version();

    let ip = std::env::var("SERVICE_HOST").unwrap();

    service::provider::run(&ip);
}
