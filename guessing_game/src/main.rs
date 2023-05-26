use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number); //秘密の数字は次の通り: {}

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // 予測と秘密の数字を比較して、それらが等しいかどうかを確認する
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  //小さすぎ！
            Ordering::Greater => println!("Too big!"), //大きすぎ！
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
