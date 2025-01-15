use scraper::{Html, Selector};
use std::error::Error;

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
