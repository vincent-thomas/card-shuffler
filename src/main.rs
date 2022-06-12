mod strategies;

pub use strategies::{split_half, two_stack};

fn main() {
    match two_stack::shuffle(&vec![1, 2, 3, 4], &1) {
        Result::Ok(result) => println!("{:?}", result),
        Result::Err(error) => println!("{:?}", error),
    };
    match split_half::split_half(&mut vec![1, 2]) {
        Ok(result) => println!("{:?}", result),
        Err(error) => println!("{:?}", error),
    };
}
