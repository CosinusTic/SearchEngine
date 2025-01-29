use scraper::{Html, Selector};
use std::error::Error;
use std::collections::HashSet;

pub fn scrape_html_element_in_url(url: &str, html_el: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut elements: Vec<String> = Vec::<String>::new();
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    let document = Html::parse_document(&html);

    // select the HTML elements
    let html_element_selector = Selector::parse(html_el).unwrap();
    let html_elements = document.select(&html_element_selector);

    // iterate over the selected HTML elements
    for html_element in html_elements {
        let parsed_html_element = html_element.text().collect::<String>();
        elements.push(parsed_html_element);
    }

    Ok(elements)
}

fn is_href_valid(href: &str) -> bool {
    href.starts_with("http://") || href.starts_with("https://")
}

pub fn get_html(url: &str) -> Result<Html, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    Ok(Html::parse_document(&html))
}

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



