#[derive(Debug)]
struct Dove;

struct Duck;

trait Tweet {
    fn tweet(&self);

    fn tweet_twice(&self) {
        self.tweet();
        self.tweet();
    }
}

impl Tweet for Duck {
    fn tweet(&self) {
        // println!("Coo")
    }
}

impl Tweet for Dove {
    fn tweet(&self) {
        // println!("Quack");
        println!("{:#?}", self)
    }
}

fn main() {
    let dove = Dove {};
    dove.tweet();
    dove.tweet_twice()
}
