// So we’re doing something complicated here, long enough that we need
// multiple lines of comments to do it! Whew! Hopefully, this comment will
// explain what’s going on.

fn main() {
    println!("Hello, world!");

    another_function(); //  You can call functions above the actual functions

    number_functions(20);

    print_labeled_measurement(10, 'Z');

    let y = {
        let x = 5;
        x + 2
    };

    println!("The value of y is: {y}");

    let b = seven();

    println!("The value of b is: {b}");

    let x = plus_one(8);

    println!("The value of x is {x}");
}

fn another_function() {
    println!("Another function.");
}

fn number_functions(x: i32) {
    println!("The value of x is {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value} {unit_label}");
}

fn seven() -> i32 {
    7
}

fn plus_one(x: i32) -> i32 {
    x + 1
}