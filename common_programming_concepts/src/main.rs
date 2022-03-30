use std::io;
fn main() {

    //mutable vairable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //shadowing variable
    let y = 5;
    let y = y + 1;

    println!("The value of y is: {}", y);

    //constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    // numerical operations
    let sum = 5 + 10;

    let subtraction = 91.4 - 4.2;

    let product = 7 * 12;

    let quotient = 54.7 / 32.2;
    let floored = 2 / 3;

    let remainder = 43 % 5;

    println!("Sum: {}, Subtraction: {}, product: {}, quotient: {}, floored: {}, remainder: {}", sum, subtraction, product, quotient, floored, remainder);

    // booleans
    let t = true;
    let f: bool = false;

    // char type (ANy UTF-8 encoded char)
    let c = 'z';

    // Tuple and pattern matching
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (first, second, third) = tup;

    println!("The value of second i: {}", second);

    // arrays

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
