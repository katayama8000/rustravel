#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn show(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let penny: u32 = value_in_cents(Coin::Penny);
    println!("{}", penny);
    let nickel: u32 = value_in_cents(Coin::Nickel);
    println!("{}", nickel);
    let quarter: u32 = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("{}", quarter);
    let five: Option<i32> = Some(5);
    // someから値を取り出す
    let some_value: i32 = five.unwrap();
    // matchで値を取り出す
    let some_value: i32 = match five {
        Some(i) => i,
        None => 0,
    };
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);
    println!("{:?}, {:?}, {:?}", five, six, none);
    let some_five: Option<i32> = Some(5);
    println!("{:?}", some_five);
    let val: i32 = add(1, 2);
    println!("{}", val);

    let mut message: Message = Message::Quit;
    message.show();
    message = Message::Move { x: 1, y: 2 };
    message.show();
    message = Message::Write(String::from("hello"));
    message.show();
    message = Message::ChangeColor(1, 2, 3);
    message.show();

    let mut maybe_number: Option<i32> = Some(1);
    println!("{:?}", maybe_number);
    maybe_number = None;
    println!("{:?}", maybe_number);

    // シャドーウィング

    let x: i32 = 5;
    let x: i32 = x + 1;

    {
        let x: i32 = x * 2;
        println!("{}", x);
    }

    println!("{}", x);

    // mut
    // 再宣言なのでOK
    let str: &str = "aaa";
    let str: usize = str.len();
    // 再代入なのでNG
    let mut str2: &str = "aaa";
    // str2 = str2.len();
}
