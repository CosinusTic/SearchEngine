mod my_io;
mod data;
mod scraping;

use my_io::user_input::*;
use std::error::Error;
use std::collections::HashSet;

fn simple_scraping_prog() -> Result<(), Box<dyn Error>> {
    let url = get_user_input("Url: ");
    println!("You entered: {}", &url);
    let tag = get_user_input("HTML tag: ");
    println!("You entered: {}", &tag);
    let elements = scraping::scrape_html_element_in_url(&url, &tag)?;
    
    for el in elements {
        println!("{:?}", el);
    }

    Ok(())
}



fn main() {
    let google_search = String::from("https://www.google.com/search?q=example&sca_esv=c7459735fc04b658&sxsrf=AHTn8zqGT-0sGAk8FJaHdx0zmT3oIWAnlg%3A1738186356673&ei=dJ6aZ7HtKO-akdUPpbSf2Qc&ved=0ahUKEwix997i8JuLAxVvTaQEHSXaJ3sQ4dUDCBI&uact=5&oq=example&gs_lp=Egxnd3Mtd2l6LXNlcnAiB2V4YW1wbGUyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQFI5AVQiwJY0ARwAXgBkAEAmAE5oAHCAaoBATS4AQPIAQD4AQGYAgWgAucBwgIHECMYsQIYJ8ICBxAAGIAEGArCAgoQIxiABBgnGIoFwgIKEAAYgAQYQxiKBcICChAAGIAEGBQYhwLCAg0QABiABBixAxiDARgKwgIFEAAYgATCAgoQABiABBixAxgKmAMA4gMFEgExIECIBgGQBhG6BgYIARABGAiSBwE1oAfdKA&sclient=gws-wiz-serp");
    let url_string = String::from("http://www.example.com");
    let score = 10;
    let mut visited: HashSet<String> = HashSet::new();
    let raw: Vec<String> = std::env::args().collect();
    if let Some(parsed_args) = data::CLIArgs::new(raw) {
        // scraping::crawl(url_string.as_str(), 0, &mut visited);
        scraping::crawl(google_search.as_str(), 0, &mut visited);
        println!("Arguments parsed: url={:?}, depth={:?}", parsed_args.url, parsed_args.crawling_depth);
    }
    else {
        println!("Arguments passed are of the wrong kind");
    }
    if let Some(url) = data::Url::new(url_string, score) {
        url.print_url(); 
    } else {
        println!("Invalid URL");
    }
}

