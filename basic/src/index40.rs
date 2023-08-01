fn main() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", arr);
    arr[0] = 2;
    println!("{:?}", arr);
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec);
    vec.push(6);
    println!("{:?}", vec);
    vec[0] = 2;
    println!("{:?}", vec);
    vec.pop();
    println!("{:?}", vec);
    let x = 5;
    let y = &x;
    println!("{:p}", y);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let a = [1, 2, 3];
    let mut iter: std::slice::Iter<'_, i32> = a.iter();
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());
    // ... and then None once it's over.
    assert_eq!(None, iter.next());

    // More calls may or may not return `None`. Here, they always will.
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
    let v1: Vec<u8> = vec![1, 2, 3];
    let v1_iter: std::slice::Iter<'_, u8> = v1.iter();
    let total: u8 = v1_iter.sum();
    assert_eq!(total, 6);
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
    for v in v1.iter() {
        println!("{}", v)
    }
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
    let vec = vec![4, 5, 6];
    let iter = vec.iter();
    for v in iter {
        println!("{}", v);
    }
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    println!("{:?}", in_my_size);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
