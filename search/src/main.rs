use common::data::{CLIArgs, Url};

fn main() {
    let score = 10;
    let raw: Vec<String> = std::env::args().collect();
    if let Some(parsed_args) = CLIArgs::new(raw) {
        println!("Arguments parsed: url={:?}, depth={:?}", parsed_args.url, parsed_args.crawling_depth);
        if let Some(url) = Url::new(parsed_args.url, score) {
            url.print_url(); 
        } else {
            println!("Invalid URL");
        }
    }
    else {
        println!("Arguments passed are of the wrong kind");
    }
}

