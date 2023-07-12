// trait

trait Human {
    fn name(&self) -> String;
    fn age(&self) -> u32;
    fn sex(&self) -> String;
}

struct Person {
    name: String,
    age: u32,
    sex: String,
}

impl Human for Person {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn age(&self) -> u32 {
        self.age
    }
    fn sex(&self) -> String {
        self.sex.clone()
    }
}

trait Animal {
    fn bow(&self) -> String;
    fn bowbow(&self) -> String;
}

struct Monkey {
    name: String,
    age: u32,
}

impl Animal for Monkey {
    fn bow(&self) -> String {
        self.name.clone()
    }

    fn bowbow(&self) -> String {
        self.age.to_string()
    }
}

trait Geometry {
    fn area(&self) -> f64;
    fn name(&self) -> &str {
        return "Geometry";
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Geometry for Rectangle {
    fn area(&self) -> f64 {
        self.width as f64 * self.height as f64
    }
    fn name(&self) -> &str {
        return "Rectangle";
    }
}

fn main() {
    let r = Rectangle {
        width: 10,
        height: 20,
    };
    println!("{} area: {}", r.name(), r.area());

    let p = Person {
        name: "John".to_string(),
        age: 20,
        sex: "male".to_string(),
    };
    println!("name: {}, age: {}", p.name(), p.age());

    let m = Monkey {
        name: "Monkey".to_string(),
        age: 10,
    };

    println!("{} age {}", m.bow(), m.bowbow());
}
