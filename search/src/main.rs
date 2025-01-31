use common::data::{CLIArgs, Url};

fn main() {
    let url_string = String::from("www.example.com");
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
}

