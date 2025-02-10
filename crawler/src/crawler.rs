use common::scraping::{get_html, is_href_valid};
use scraper::Selector;
use std::collections::HashSet;
use std::sync::mpsc::Sender;

pub fn crawl(url: &str, lim: u8, visited: &mut HashSet<String>, sender: Sender<String>) {
    if lim >= 10 || visited.contains(url) {
        return;
    }

    visited.insert(url.to_string());
    match get_html(url) {
        Ok(html) => {
            let selector = Selector::parse("a").unwrap();
            for anchor in html.select(&selector) {
                if let Some(href) = anchor.value().attr("href") {
                    if is_href_valid(href) {
                        let href_cloned = href.to_string();
                        println!("Href crawled: {}", href);

                        sender.send(href_cloned).expect("Failed to send");

                        let sender = sender.clone();
                        crawl(href, lim + 1, visited, sender);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch URL {}: {}", url, err);
        }
    }
}
