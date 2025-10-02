pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
pub trait Thh {
    fn thh(&self);
    fn not();
}
impl Thh for NewsArticle {
    fn thh(&self) {
        println!("thh");
    }
    fn not() {
        println!("not");
    }
}
pub trait Summary {
    fn summarize(&self) -> String;
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline, self.author, self.location
        )
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
