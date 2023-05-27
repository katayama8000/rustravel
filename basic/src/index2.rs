fn main() {
    let a = 10; // immutable object
                // let b = a; // copy
                // print!("{} {}", a, b); // borrow check!! - OK
                // 文字列
    let s: &str = "hello";
    println!("{}", s);
    // len
    let len: usize = s.len();
    println!("{}", len);
    // 参照
    let a_mut_ref1 = a;
    println!("{}", a_mut_ref1);
    let mut b = 20;
    println!("{}", b);
    let b_mut_ref2 = &mut b;
    let b_mut_ref3 = &mut b;
    let b_immut_ref1 = &b;

    let c = 30;
    let c_mut_ref1 = &c;
    let c_mut_ref2 = &c;

    let mut d = 40;
    let d_ref = &d;
    let d_mut_ref1 = &mut d;
    let d_mut_ref2 = &mut d;
    let d_ref2 = &d;
    println!("{}", d_ref);
}
