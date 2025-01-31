mod crawler;

use common::CLIArgs;
use std::collections::HashSet;

fn main() {
    let google_search = String::from("https://www.google.com/search?q=example&sca_esv=c7459735fc04b658&sxsrf=AHTn8zqGT-0sGAk8FJaHdx0zmT3oIWAnlg%3A1738186356673&ei=dJ6aZ7HtKO-akdUPpbSf2Qc&ved=0ahUKEwix997i8JuLAxVvTaQEHSXaJ3sQ4dUDCBI&uact=5&oq=example&gs_lp=Egxnd3Mtd2l6LXNlcnAiB2V4YW1wbGUyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyChAAGLADGNYEGEcyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyDRAAGIAEGLADGEMYigUyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQEyExAuGIAEGLADGEMYyAMYigXYAQFI5AVQiwJY0ARwAXgBkAEAmAE5oAHCAaoBATS4AQPIAQD4AQGYAgWgAucBwgIHECMYsQIYJ8ICBxAAGIAEGArCAgoQIxiABBgnGIoFwgIKEAAYgAQYQxiKBcICChAAGIAEGBQYhwLCAg0QABiABBixAxiDARgKwgIFEAAYgATCAgoQABiABBixAxgKmAMA4gMFEgExIECIBgGQBhG6BgYIARABGAiSBwE1oAfdKA&sclient=gws-wiz-serp");
    let mut visited: HashSet<String> = HashSet::new();
    let raw: Vec<String> = std::env::args().collect();
    if let Some(parsed_args) = CLIArgs::new(raw) {
        crawler::crawl(google_search.as_str(), 0, &mut visited);
        println!("Arguments parsed: url={:?}, depth={:?}", parsed_args.url, parsed_args.crawling_depth);
    }
    else {
        println!("Arguments passed are of the wrong kind");
    }
}
