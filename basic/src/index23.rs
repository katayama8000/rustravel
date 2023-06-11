pub trait Summary1 {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary1 for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    // trait
    let article1: NewsArticle = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
        hockey team in the NHL.",
        ),
    };

    let article2: NewsArticle = NewsArticle {
        headline: String::from("Japan is the best country in the world!"),
        location: String::from("Tokyo, Japan"),
        author: String::from("Taro Yamada"),
        content: String::from("Japan is the best country in the world. I love Japan."),
    };

    println!("New article available! {}", article1.summarize());
    println!("New article available! {}", article2.summarize());
}
