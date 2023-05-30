struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    //　構造体

    let mut user1: User = User {
        email: String::from("abc@example.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    let user2: User = build_user(String::from("sample@example.com"), String::from("sample"));

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
