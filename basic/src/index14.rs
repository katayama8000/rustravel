#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
        println!("{:?}", self);
    }
}

fn main() {
    let four: IpAddrKind = IpAddrKind::V4;
    let six: IpAddrKind = IpAddrKind::V6;
    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    println!("{:?}", home);

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    println!("{:?}", loopback);

    let m: Message = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_type: IpAddrKind) {
    println!("{:?}", ip_type);
}
