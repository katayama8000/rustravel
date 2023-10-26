use std::collections::HashMap;

fn main() {
    let vec1 = vec![0, 1, 2, 3];
    let vec11 = vec1.iter().map(|x| x * 2).collect::<Vec<i32>>();
    println!("{:?}", vec1);
    println!("{:?}", vec11);

    let mut vec2: Vec<i32> = vec![0, 1, 2, 3];
    let vec22 = vec2.iter_mut().map(|x| *x * 2).collect::<Vec<i32>>();
    println!("{:?}", vec2);
    println!("{:?}", vec22); // [0, 2, 4, 6, 8]

    let vec3 = vec![0, 1, 2, 3];
    let vec33 = vec3
        .clone()
        .into_iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
    println!("{:?}", vec3);
    println!("{:?}", vec33); // [0, 2, 4, 6, 8]

    let vec4 = vec![0, 1, 2, 3];
    let one = vec4.iter().next().unwrap();
    println!("{}", one);
    println!("{}", vec4[1]);

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    println!("{:?}", v1_iter);

    assert_eq!(v1_iter.next(), Some(&1));
    println!("{:?}", v1_iter);
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let count: usize = scores.iter().count();
    println!("{}", count);

    let mut v = vec![1, 2, 3];
    let v_iter = v.iter(); // 不変の参照を生成
    let v_mut_iter = v.iter_mut(); // 不変の参照を生成
    let v_into_iter = v.into_iter(); // 所有権を奪う
    let mut v = vec![1, 2, 3];
    for num in v.iter_mut() {
        *num *= 2; // 各要素を2倍に変更
    }
    println!("{:?}", v);
    let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.iter().map(|i| i * 3).collect::<Vec<i32>>();
    println!("{:#?}", vec2);
    let vec3 = vec1.into_iter();
}
