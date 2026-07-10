use traits::articles::{
    NewsArticle, SocialMediaPost, Summary, alert, breaking_news, new_post, notify,
};

fn main() {
    // News article

    let news1 = NewsArticle {
        headline: "Apollo 11".to_string(),
        location: "Moon".to_string(),
        author: String::from("Neil Armstrong"),
        content: String::from("He is the first man to ever set foot on the moon"),
    };

    let news1_summary = news1.summarize();
    let news1_summary_author = news1.summarize_author();

    println!("{}", news1_summary);
    println!("{}", news1_summary_author);

    // Social media post

    let social1 = SocialMediaPost {
        username: "elonmusk".to_string(),
        content: "Let's build a rocket to go to Mars.".to_string(),
        reply: false,
        repost: false,
    };

    println!("Summary - {}", social1.summarize());
    println!("Author - {}", social1.summarize_author());

    // Using functions that implement Summary
    notify(&news1);
    breaking_news(&social1);
    alert(&social1);

    // Calling a function that returns a type implementing Summary trait.
    let social2 = new_post("I went first to the space.".to_string());
    println!("{}", social2.summarize());
}
