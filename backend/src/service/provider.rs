use log;

use crate::core;

pub trait Interface {
    fn run(&self);
}

pub fn run(host: &str) {
    log::debug!("create new service");
    super::service::new(core::provider::new(host)).run();
}
