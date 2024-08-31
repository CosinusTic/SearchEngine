mod scraping;
mod user_interaction;

use scraping::scraping::scrape_html_element_in_url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = user_interaction::user_input::get_user_input("Url: ");
    println!("You entered: {}", &url);
    let tag = user_interaction::user_input::get_user_input("HTML tag: ");
    println!("You entered: {}", &tag);
    //let url = "https://www.scrapethissite.com/pages/simple/";
    //let html_el = "h3";
    let elements = scrape_html_element_in_url(&url, &tag);

    for element in elements.iter().enumerate(){
        println!("{:?}", element);
    }

    Ok(())
}