use reqwest::{blocking::Client as SyncClient, Client as AsyncClient};
use select::document::Document;
use select::predicate::Name;
use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::task::spawn;
use url::Url;

pub enum HyperSeekMode {
    Sync,
    Async,
}

pub struct HyperSeek {
    visited_urls: Arc<Mutex<HashSet<String>>>,
    urls_to_download: Arc<Mutex<VecDeque<String>>>,
    max_depth: u32,
    mode: HyperSeekMode,
}
