fn next_birthday1(current_age: Option<u8>) -> Option<String> {
    // If `current_age` is `None`, this returns `None`.
    // If `current_age` is `Some`, the inner `u8` gets assigned to `next_age`
    // `current_age`が`None`の場合、`None`を返す。
    // `current_age`が`Some`の場合、内部の`u8`型の値が`next_age`に代入される。
    let next_age: u8 = current_age? + 1;
    println!("next age is {}", next_age);
    Some(format!("Next year I will be {}", next_age))
}

fn next_birthday2(current_age: Option<u8>) -> Option<String> {
    let next_age: u8 = match current_age {
        Some(age) => age + 1,
        None => return None,
    };
    println!("next age is {}", next_age);
    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: Option<u32>,
}

impl Person {
    // Gets the area code of the phone number of the person's job, if it exists.
    // その人の市外局番が存在する場合、取得する。
    fn work_phone_area_code(&self) -> Option<u8> {
        // This would need many nested `match` statements without the `?` operator.
        // It would take a lot more code - try writing it yourself and see which
        // is easier.
        // `?`がなければ、多くのネストされた`match`文を必要とするため、より長いコードとなる。
        // 実際に書いて、どちらの方が簡単か確かめてみましょう。
        self.job?.phone_number?.area_code
    }

    fn work_phone_number(&self) -> Option<u32> {
        self.job?.phone_number?.number
    }
}

fn main() {
    let current_age: Option<u8> = Some(10);
    next_birthday1(current_age);

    let current_age: Option<u8> = Some(12);
    next_birthday2(current_age);

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: Some(429222222),
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
    assert_eq!(p.work_phone_number(), Some(429222222))
}
