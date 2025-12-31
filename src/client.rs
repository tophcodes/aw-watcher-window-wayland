use aw_client_rust::blocking::AwClient;
use anyhow::Result;
use gethostname::gethostname;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BucketType {
    Window,
    AFK,
}

pub struct Client {
    boxed: Box<AwClient>,
    buckets: HashMap<BucketType, String>,
}

impl Client {
    pub fn new(host: &str, port: u16, client_name: &str) -> Result<Self> {
        let client = AwClient::new(host, port, client_name)?;
        let hostname = gethostname().into_string().unwrap();
        let mut buckets = HashMap::new();

        let window_bucket = format!("aw-watcher-window_{}", hostname);
        let afk_bucket = format!("aw-watcher-afk_{}", hostname);

        buckets.insert(BucketType::Window, window_bucket);
        buckets.insert(BucketType::AFK, afk_bucket);

        Ok(Self {
            boxed: Box::new(client),
            buckets,
        })
    }

    pub fn create_bucket_simple(&self, bucket_type: BucketType, type_name: &str) -> Result<()> {
        let bucket_name = self.buckets.get(&bucket_type)
            .ok_or_else(|| anyhow::anyhow!("Bucket type not found"))?;
        self.boxed.create_bucket_simple(bucket_name, type_name)
    }

    pub fn heartbeat(&self, bucket_type: BucketType, event: &aw_client_rust::Event, pulsetime: f64) -> Result<()> {
        let bucket_name = self.buckets.get(&bucket_type)
            .ok_or_else(|| anyhow::anyhow!("Bucket type not found"))?;
        self.boxed.heartbeat(bucket_name, event, pulsetime)
    }
}
