use std::fs::File;
fn main() {
    #![allow(dead_code)]

    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    // Peeling food. If there isn't any, then return `None`.
    // Otherwise, return the peeled food.
    // 食べ物の皮をむく。存在しない場合は単純に`None`を返す。
    // そうでなければ皮を向いた食べ物を返す。
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    // Chopping food. If there isn't any, then return `None`.
    // Otherwise, return the chopped food.
    // 上と同じように、食べ物を切る前に、皮を向いた食べ物の有無を知る必要がある。
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
    // 上のチェックと同様だが`match`の代わりに`map()`を使用している。
    // fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    //     chopped.map(|Chopped(food)| Cooked(food))
    // }

    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        match chopped {
            Some(Chopped(food)) => Some(Cooked(food)),
            None => None,
        }
    }

    // A function to peel, chop, and cook food all in sequence.
    // We chain multiple uses of `map()` to simplify the code.
    // 複数の`map()`をチェインさせて、上のプロセスをシンプルにすることもできる。
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(|f| Peeled(f))
            .map(|Peeled(f)| Chopped(f))
            .map(|Chopped(f)| Cooked(f))
    }

    // Check whether there's food or not before trying to eat it!
    // 食べる前に、食べ物の有無をチェックするのは大事ですよね!
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm. I love {:?}", food),
            None => println!("Oh no! It wasn't edible."),
        }
    }

    println!("xxx");
    let apple: Option<Food> = Some(Food::Apple);
    let carrot: Option<Food> = Some(Food::Carrot);
    let potato: Option<Food> = None;

    let cooked_apple: Option<Cooked> = cook(chop(peel(apple)));
    // let cooked_carrot: Option<Cooked> = cook(chop(peel(carrot)));
    let cooked_carrot: Option<Cooked> = process(carrot);
    // Let's try the simpler looking `process()` now.
    // よりシンプルな見た目の`process()`を使用する。
    let cooked_potato: Option<Cooked> = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);

    let opt: Option<&str> = Some("hello");
    opt.expect("err");
    let f = File::open("hello.txt");
    f.expect("this is a error");
    let v: Result<File, std::io::Error> = File::open("error.txt");
    // v.expect("this is a error");
    match v {
        Ok(file) => {
            // ファイルが正常に開かれた場合の処理
            println!("ファイルを正常に開きました。{:#?}", file);
        }
        Err(e) => {
            // ファイルが開けなかった場合の処理
            println!("ファイルを開けませんでした。エラー: {:?}", e);
        }
    }
}
