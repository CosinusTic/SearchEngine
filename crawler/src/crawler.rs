use scraper::Selector;
use std::collections::HashSet;
use common::scraping::{ get_html, is_href_valid };

pub fn crawl(url: &str, lim: u8, visited: &mut HashSet<String>) {
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
                        println!("Href crawled: {}", href);
                        crawl(href, lim + 1, visited);
                    }
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to fetch URL {}: {}", url, err);
        }
    }
}

