struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

struct Human {
    name: String,
    age: u32,
}

fn main() {
    let mut counter: Counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);

    let human = Human {
        name: "honoka".to_string(),
        age: 10,
    };
    println!("{}{}", human.age, human.name);

    let a = [1, 2, 3];
    assert_eq!(a.iter().count(), 3);

    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().count(), 5);

    let a = [1, 2, 3];
    assert_eq!(a.iter().last(), Some(&3));

    let a = [1, 2, 3, 4, 5];
    assert_eq!(a.iter().last(), Some(&5));

    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().zip(a2.iter());

    assert_eq!(iter.next(), Some((&1, &4)));
    assert_eq!(iter.next(), Some((&2, &5)));
    assert_eq!(iter.next(), Some((&3, &6)));
    assert_eq!(iter.next(), None);

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    for (v1, v2) in vec1.iter().zip(vec2.iter()) {
        println!("v1: {}, v2: {}", v1, v2);
    }
}
