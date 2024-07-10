use log;

use crate::core::provider;

struct Service {
    _core: Box<dyn provider::CoreInterface>,
}

impl super::provider::Interface for Service {
    fn run(&mut self) {
        log::debug!("run service");
        self._core.run();
    }
}

pub fn new(core: Box<dyn provider::CoreInterface>) -> Box<dyn super::provider::Interface> {
    Box::new(Service { _core: core })
}
