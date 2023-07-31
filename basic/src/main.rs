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
}
