use log;

use crate::core;

pub fn new() -> Box<dyn super::service::Interface> {
    log::debug!("create new service");
    let core = core::provider::new();
    super::service::new(core)
}
