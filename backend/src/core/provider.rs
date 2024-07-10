use log;

pub trait CoreInterface {
    fn init(&mut self, host: &str);
    fn run(&mut self);
}

pub fn new() -> Box<dyn CoreInterface> {
    log::debug!("create new core");

    let nthreads = std::thread::available_parallelism();
    let mut pool = super::pool::new(nthreads.unwrap().get());

    pool.init();

    super::core::new(pool)
}
