extern crate env_logger;
extern crate log;

use chrono::Local;
use dotenv::dotenv;
use service::service::Interface;

mod core;
mod service;

fn version() {
    dotenv().ok();

    let version = env!("CARGO_PKG_VERSION");
    let timestamp = Local::now().format("%d/%m/%Y %H:%M");

    log::info!("Application version v.{} from {}", version, timestamp);
}

fn main() {
    env_logger::init();

    version();

    service::provider::new().init().run();
}
