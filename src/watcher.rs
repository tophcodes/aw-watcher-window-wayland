use aw_client_rust::blocking::AwClient;
use anyhow::Result;
use gethostname::gethostname;
use std::collections::HashMap;

pub enum BucketType {
    Window,
    AFK,
}

struct Client {
    boxed: Box<AwClient>,
    buckets: HashMap<BucketType, String>,
}

impl Client {
    pub fn new(host: &str, port: u16, client_name: &str) -> Result<Self> {
        let client = AwClient::new(host, port, client_name)?;
        let hostname = gethostname().into_string().unwrap();

        buckets = HashMap::new();

        Ok(Self {
            boxed: Box::new(client),
            buckets,
        })
    }
}
