fn main() {
    // 参照はずし
    let mut str = "hello";
    println!("{}", str);
    str = "world";
    println!("{}", str);

    let mut num = 0;
    let ref_num = &mut num;
    *ref_num = 20;

    let str = String::from("abc");
    let str_ref = &String::from("def");
    if str == *str_ref {
        println!("equal");
    } else {
        println!("not equal");
    }
    let mut str_ref1: &mut String = &mut String::from("def");
    str_ref1.push('z');

    let mut num: i32 = 30;
    let ref_num: &mut i32 = &mut num;
    *ref_num = 20;

    let mut c: char = 'c';
    let ref_c = &mut c;
    *ref_c = 'z';

    let mut strr: String = String::from("aaa");
    let ref_strr: &mut String = &mut strr;
    *ref_strr = String::from("bbbbb");
    println!("{}", strr);
}
