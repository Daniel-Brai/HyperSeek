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

impl HyperSeek {
    pub fn create(seed_urls: Vec<String>, max_depth: u32, mode: HyperSeekMode) -> HyperSeek {
        let visited_urls = Arc::new(Mutex::new(HashSet::new()));
        let urls_to_download = Arc::new(Mutex::new(VecDeque::from(seed_urls)));

        HyperSeek {
            visited_urls,
            urls_to_download,
            max_depth,
            mode,
        }
    }

    pub fn crawl(&mut self) {
        match self.mode {
            HyperSeekMode::Sync => self.crawl_sync(),
            HyperSeekMode::Async => self.crawl_async(),
        }
    }

    fn crawl_sync(&mut self) {
        // implement hyperseek web crawling synchronously
    }

    fn crawl_async(&mut self) {
        // implement hyperseek web crawling asynchronously
    }
}
