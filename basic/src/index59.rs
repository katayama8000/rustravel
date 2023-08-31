#[derive(Default)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2, z: 3 };
    println!("p1: x={}, y={}, z={}", p1.x, p1.y, p1.z);

    let p2 = Point { x: 4, ..p1 };
    println!("p2: x={}, y={}, z={}", p2.x, p2.y, p2.z);

    let p3: Point = Default::default();
    println!("p3: x={}, y={}, z={}", p3.x, p3.y, p3.z);
}
