use ::std::cmp::PartialOrd;

fn largest_number<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest: T = list[0];

    for &number in list {
        if number > largest {
            largest = number;
        }
    }

    largest
}
fn main() {
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let mut largest: i32 = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    let result: i32 = largest_number(&number_list);
    println!("The largest number is {}", result);

    let number_list: Vec<f64> = vec![34.0, 50.0, 25.0, 100.1, 65.0];
    let result: f64 = largest_number(&number_list);
    println!("The largest number is {}", result);
}
