pub mod io;
pub use io::get_user_input;

pub mod scraping;
pub use scraping::{get_html, scrape_html_element_in_url, is_href_valid };

pub mod data;
pub use data::{ Url, CLIArgs };
