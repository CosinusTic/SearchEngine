pub struct Url {
    url: String,
    score: i32
}

impl Url
{
    pub fn new(url: String, score: i32) -> Option<Url> {
        if !url.contains("http") || !url.contains("www") {
            return None;
        }
        
        Some(Url {
            url,
            score
        })
    }

    pub fn print_url(&self) {
        println!("url: {}, score: {}", self.url, self.score);
    }
}

pub struct CLIArgs{
    pub url: String,
    pub crawling_depth: i32,
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
