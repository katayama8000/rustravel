pub mod lib1;
pub mod three;

use three::return_number;

fn main() {
    let some_u8_value: Option<i32> = Some(4);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("nothing")
    }
    lib1::hello();
    let num: usize = return_number::return_number();
    println!("The number is {}", num);
}
