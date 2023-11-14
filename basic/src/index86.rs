//https://doc.rust-jp.rs/book-ja/ch10-02-traits.html
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
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

pub fn notify<T: Summary>(item: &T) {
    // 速報！ {}
    println!("Breaking news! {}", item.summarize());
}

// 継承
trait Test: Summary {
    fn test(&self) -> String {
        format!("test")
    }
}

impl Test for Tweet {
    fn test(&self) -> String {
        format!("test")
    }
}

fn main() {
    let tweet = Tweet {
        username: "katayama".to_string(),
        content: "I love you".to_string(),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize_author());

    notify(&tweet);
}
