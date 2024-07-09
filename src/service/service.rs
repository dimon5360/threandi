use log;

use crate::core::core;

pub trait Interface {
    fn init(&self) -> &Service;
    fn run(&self);
}

pub struct Service {
    _core: Box<dyn core::Interface>,
}

impl Interface for Service {
    fn init(&self) -> &Service {
        log::debug!("init service");
        self._core.init();
        self
    }

    fn run(&self) {
        log::debug!("run service");
        self._core.run();
    }
}

pub fn new(core: Box<dyn core::Interface>) -> Box<dyn Interface> {
    Box::new(Service { _core: core })
}
