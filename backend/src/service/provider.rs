use log;

use crate::core;

pub trait Interface {
    fn run(&mut self);
}

pub fn new(host: &str) -> Box<dyn Interface> {
    log::debug!("create new service");

    let mut core = core::provider::new();
    core.init(host);

    super::service::new(core)
}
