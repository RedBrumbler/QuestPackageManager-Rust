use std::{
    lazy,
    time::Duration,
};

use crate::data::config::Config;

static AGENT: lazy::SyncOnceCell<reqwest::blocking::Client> = lazy::SyncOnceCell::new();

pub fn get_agent() -> &'static reqwest::blocking::Client {
    AGENT.get_or_init(|| {
        reqwest::blocking::ClientBuilder::new()
            .connect_timeout(Duration::from_millis(
                Config::read_combine().timeout.unwrap(),
            ))
            .user_agent(format!("questpackagemanager-rust/{}", env!("CARGO_PKG_VERSION")).as_str())
            .build()
            .expect("Client agent was not buildable")
    })
}
