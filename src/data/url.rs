pub struct Url {
    url: String,
    score: i32
}

impl Url
{
    pub fn new(url: String, score: i32) -> Option<Url>
    {
        if !url.contains("http") || !url.contains("www")
        {
            return None;
        }
        
        Some(Url {
            url: url,
            score: score
        })
    }

    pub fn print_url(&self) {
        println!("url: {}, score: {}", self.url, self.score);
    }
}
