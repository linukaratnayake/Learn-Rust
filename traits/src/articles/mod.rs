pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct SocialMediaPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialMediaPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!(
            "Read more from @{}, Content - {}",
            self.username, self.content
        )
    }
}

// All the following function signatures are the same.

pub fn notify(item: &impl Summary) {
    println!("Notification - {}", item.summarize_author());
}

pub fn breaking_news<T: Summary>(item: &T) {
    println!("Breaking news! - {}", item.summarize_author());
}

pub fn alert<T>(item: &T)
where
    T: Summary, // With `+` we can add more traits.
{
    println!("Alert = {}", item.summarize());
}
