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

pub struct CLIArgs{
    url: String,
    crawling_depth: i32,
}

impl CLIArgs {
    pub fn new(args: Vec<String>) -> Option<CLIArgs> {
        if args[0].is_empty() || args[1].is_empty() {
            return None;
        }

        Some(CLIArgs {
            url: String::from(&args[1]),
            crawling_depth: args[2].parse::<i32>().unwrap(),
        })
    }
}


fn main() {
    let url_string = String::from("http://www.example.com");
    let score = 10;

    let raw: Vec<String> = std::env::args().collect();
    if let Some(parsed_args) = CLIArgs::new(raw) {
        println!("Arguments parsed: url={:?}, depth={:?}", parsed_args.url, parsed_args.crawling_depth);
    }
    else {
        println!("Arguments passed are of the wrong kind");
    }
    if let Some(url) = Url::new(url_string, score) {
        url.print_url(); 
    } else {
        println!("Invalid URL");
    }

    let out = simple_scraping_prog();


    match out {

        Ok(out) => println!("Successfully retrieved html elements, returned"),
        Err(out) => println!("Failed to retrieve html elelents"),
    };
}

