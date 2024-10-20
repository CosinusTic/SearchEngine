mod scraping;
mod my_io;
mod data;

use scraping::scraping::scrape_html_element_in_url;
use my_io::user_input::*;
use data::url::Url;
use std::error::Error;

fn simple_scraping_prog() -> Result<(), Box<dyn Error>> {
    let url = get_user_input("Url: ");
    println!("You entered: {}", &url);
    let tag = get_user_input("HTML tag: ");
    println!("You entered: {}", &tag);
    let elements = scrape_html_element_in_url(&url, &tag)?;
    
    for el in elements {
        println!("{:?}", el);
    }

    Ok(())
}

fn main() {
    let url_string = String::from("http://www.example.com");
    let score = 10;

    if let Some(url) = Url::new(url_string, score) {
        url.print_url(); 
    } else {
        println!("Invalid URL");
    }
}
