struct Human {
    name: String,
    age: u32,
    height: u32,
    weight: u32,
}

impl Human {
    fn new(name: String, age: u32, height: u32, weight: u32) -> Self {
        Self {
            name,
            age,
            height,
            weight,
        }
    }

    fn print(&self) {
        println!(
            "Name: {}, Age: {}, Height: {}, Weight: {}",
            self.name, self.age, self.height, self.weight
        );
    }
}

struct Color(i32, i32, i32);

impl Color {
    fn print(&self) {
        println!("R: {}, G: {}, B: {}", self.0, self.1, self.2);
    }
}

fn main() {
    let human: Human = Human::new(String::from("John"), 20, 170, 60);
    human.print();

    let tom: Human = Human {
        name: String::from("Tom"),
        age: 25,
        ..human
    };

    tom.print();

    let color: Color = Color(255, 255, 255);
    color.print();
}
