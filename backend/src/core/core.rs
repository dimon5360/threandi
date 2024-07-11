use log;
use std::net::TcpListener;
use std::sync::Arc;

use crate::api::{endpoint::Endpoint, gateway};

struct Core {
    _pool: super::pool::ThreadPool,
    _host: String,
    _endpoints: Vec<Endpoint>,
}

impl super::provider::CoreInterface for Core {
    fn run(&self) {
        let gateway = gateway::new();

        // TODO: create gateway,
        // execute max connection handers
        // move gateway clones to handlers

        let arc = Arc::new(gateway);

        let listener = TcpListener::bind(&self._host).unwrap();

        log::debug!("listening {} ... ", self._host);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            let clone = arc.clone();

            self._pool.execute(move || {
                clone.handle_connection(stream);
            })
        }

        log::debug!("run core");
    }
}

pub fn new(
    pool: super::pool::ThreadPool,
    host: &str,
    endpoints: Vec<Endpoint>,
) -> Box<dyn super::provider::CoreInterface> {
    Box::new(Core {
        _pool: pool,
        _host: host.to_string(),
        _endpoints: endpoints,
    })
}
