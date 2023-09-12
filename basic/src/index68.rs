trait SayAge {
    fn say_age(&self);
}

trait SayName: SayAge {
    fn say_name(&self);
}

struct Person {
    age: i32,
    name: String,
}

impl SayName for Person {
    fn say_name(&self) {
        println!("{}", self.age)
    }
}

impl SayAge for Person {
    fn say_age(&self) {
        println!("{}", self.name)
    }
}

fn main() {
    let person = Person {
        age: 20,
        name: "aaa".to_string(),
    };

    person.say_age();
    person.say_name();
}
