struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn xy(self) -> (T, T) {
        (self.x, self.y)
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let a = 1;
    let b = 2;
    let c = add(a, b);
    println!("{} + {} = {}", a, b, c);

    let point = Point::<i32> { x: 1, y: 2 };
    println!("point: {:?}", point.xy());

    let point = Point::<f64> { x: 3.1, y: 5.0 };
    println!("point: {:?}", point.xy());

    let coin = Coin::Penny;
    println!("coin: {}", value_in_cents(coin));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);
}

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
