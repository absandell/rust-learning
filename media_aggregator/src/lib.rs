pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!(
            "(Read more from {}...)",
            self.summarize_author()
        )
    }
    
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

/* Trait bound form of the above */
/*pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}*/

/* News Article */
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({})",
            self.headline,
            self.author,
            self.location
        )
    }

    fn summarize_author(&self) -> String {
        self.author.clone()
    }

}

/* Tweet */
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}", 
            self.username, 
            self.content
        )
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}