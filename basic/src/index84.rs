fn main() {
    let it = Iter { current: 0, max: 5 };
    // println!("{:?}", it);
    // println!("{:?}", it.next().unwrap());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // println!("{:?}", it.next());
    // for i in it {
    //     println!("{}", i);
    // }
    let arr = [1, 2, 3, 4, 5];
    let iter = arr.iter();
    for i in iter {
        println!("{}", i);
    }

    let h = Human::new("John", 32);
}

#[derive(Debug)]
struct Iter {
    current: usize,
    max: usize,
}

struct Human {
    name: String,
    age: u32,
}

trait Animal {
    fn eat(&self);
    fn sleep(&self);
}

impl Animal for Human {
    fn eat(&self) {
        println!("{} is eating", self.name);
    }

    fn sleep(&self) {
        println!("{} is sleeping", self.name);
    }
}

impl Human {
    fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

impl Iterator for Iter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.current += 1;
        if self.current - 1 < self.max {
            Some(self.current - 1)
        } else {
            None
        }
    }
}
