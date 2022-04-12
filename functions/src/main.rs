fn main() {
    print_labeled_measurement(5, 'h');

    // Statements and Expressions
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {}", y);

    //Returning Values
    let five = five();

    println!("The value of five is: {}", five);

    let add_one = plus_one(5);

    println!("The value of add_one is: {}", add_one);
}

fn print_labeled_measurement(value: i32, unit_lavel: char) {
    println!("The measurement is {}{}", value, unit_lavel);
}

fn five() -> i32 {
    5
}

fn plus_one(value: i32) -> i32 {
    value + 1
}