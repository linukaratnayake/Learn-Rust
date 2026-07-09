use traits::articles::{NewsArticle, SocialMediaPost, Summary};

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
}
