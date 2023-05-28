fn main() {
    // mutable immutable
    // mutable
    let mut x = 5;
    // 参照
    let ref_x = &x;
    // 参照の参照
    let ref_ref_x = &ref_x;
    let y = &x;
    println!("{} {} {} {}", x, ref_x, ref_ref_x, y);
    // --------------------------------------------------
    // immutable
    let z = 10;
    let ref_z = &z;
    let ref_ref_z = &ref_z;
    let w = &z;
    println!("{} {} {} {}", z, ref_z, ref_ref_z, w);
}
