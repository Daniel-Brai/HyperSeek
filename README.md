# HyperSeek
A powerful and efficient web crawling library written in Rust that allows you to crawl web pages, extract URLs, and traverse the web with ease. 

## Features
The main purpose of HyperSeek is to enable fast and efficient search engine indexing.
Other feaures include:
1. **Crawling Metadata**: Collect and store additional metadata about crawled pages, such as HTTP response status codes, content type that mostly HTML for now with plans for other content types in the future, last-modified timestamps, or page sizes. This information can be useful for analysis and understanding the crawled websites.

2. **Politeness and Respectful Crawling**: Implement mechanisms to respect website policies such as robots.txt files and crawl delay. This ensures that your crawler behaves respectfully and avoids overloading servers with excessive requests.

3. **Parallel Processing**: Enable concurrent processing of multiple URLs to improve crawling speed and efficiency. Utilize Rust's concurrency primitives to efficiently distribute crawling tasks across multiple threads or even across multiple machines.

4. **Customizable Crawling Rules**: Allow users to define custom rules for crawling, such as specifying which domains to crawl, setting depth limits, or filtering URLs based on patterns or criteria.

## Features Todo
- [x] Implement Synchronous Web Crawling
- [ ] Implement Asynchronous Web Crawling
- [ ] Implement Robust Page Metadata gathering
- [ ] Implement interface for user extensible rules
