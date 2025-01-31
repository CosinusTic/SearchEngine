use common::data;
use common::scraping;

pub fn simple_scraping_prog() -> Result<(), Box<dyn Error>> {
    let url = data::get_user_input("Url: ");
    println!("You entered: {}", &url);
    let tag = data::get_user_input("HTML tag: ");
    println!("You entered: {}", &tag);
    let elements = scraping::scrape_html_element_in_url(&url, &tag)?;
    
    for el in elements {
        println!("{:?}", el);
    }

    Ok(())
}

