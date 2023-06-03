use reqwest::{blocking::Client as SyncClient, Client as AsyncClient};
use select::document::Document;
use select::predicate::Name;
use std::collections::{HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use tokio::task::spawn;
use url::Url;

pub enum HyperSeekMode {
    Async,
    Sync,
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
            HyperSeekMode::Async => self.crawl_async(),
            HyperSeekMode::Sync => self.crawl_sync(),
        }
    }

    fn get_next_url(&mut self) -> Option<String> {
        self.urls_to_download.lock().unwrap().pop_front()
    }

    async fn download_page_async(
        &self,
        client: &AsyncClient,
        url: &str,
    ) -> Result<String, reqwest::Error> {
        client.get(url).send().await?.text().await
    }

    fn download_page_sync(&self, client: &SyncClient, url: &str) -> Result<String, reqwest::Error> {
        client.get(url).send()?.text()
    }

    fn get_url_depth(&self, url: &str) -> u32 {
        let parsed_url = Url::parse(url).expect("Failed to parse URL");
        parsed_url
            .path_segments()
            .map(|segments| segments.count() as u32)
            .unwrap_or(0)
    }

    fn extract_urls(&self, page_content: &str) -> Vec<String> {
        Document::from(page_content)
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .map(|href| href.to_string())
            .collect()
    }

    fn crawl_sync(&mut self) {
        // implement hyperseek web crawling synchronously
        let client = SyncClient::new();
        let mut urls_to_add = Vec::new();

        while let Some(url) = self.get_next_url() {
            if let Ok(page_content) = self.download_page_sync(&client, &url) {
                let extracted_urls = self.extract_urls(&page_content);

                self.visited_urls.lock().unwrap().insert(url.clone());

                for extracted_url in extracted_urls {
                    let depth = self.get_url_depth(&url) + 1;
                    if !self.visited_urls.lock().unwrap().contains(&extracted_url)
                        && depth <= self.max_depth
                    {
                        urls_to_add.push(extracted_url);
                    }
                }
            }
        }

        self.urls_to_download.lock().unwrap().extend(urls_to_add);
    }

    fn crawl_async(&mut self) {
        // implement hyperseek web crawling asynchronously
    }
}
