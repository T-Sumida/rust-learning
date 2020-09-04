pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // {}さんからもっと読む
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify<T: Summary>(item: T) {
    // 新ニュース！ {}
    println!("Breaking news! {}", item.summarize());
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
