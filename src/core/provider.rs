use log;

pub fn new() -> Box<dyn super::core::Interface> {
    log::debug!("create new core");
    super::core::new()
}
