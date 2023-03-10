fn main() {
    println!("Test");
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
   fn summarize(&self) -> String {
       format!("{}, by {} ({})", self.headline, self.author, self.location)
   } 
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String {
       String::from("TEst") 
    }
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news: {}", item.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("temp"),
        reply: false,
        retweet: false,
    }
}
