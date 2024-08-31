use scraper::{Html, Selector};

pub fn scrape_html_element_in_url<'a>(url: &str, html_el: &'a str) -> Result<Vec<String>, Box<dyn std::error::Error + 'a>> {
    let mut elements: Vec<String> = Vec::new();
    let response = reqwest::blocking::get(url)?;
    let html = response.text()?;
    let document = Html::parse_document(&html);

    // select the HTML elements
    let html_element_selector = Selector::parse(html_el)?;
    let html_elements = document.select(&html_element_selector);

    // iterate over the selected HTML elements
    for html_element in html_elements {
        let parsed_html_element = html_element.inner_html();
        elements.push(parsed_html_element);
    }

    Ok(elements)
}