use std::io;


fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("There is {} Seconds in three hours", THREE_HOURS_IN_SECONDS);

    let y = 1_000;

    let y = y + 12;

    {
        let y = y * 23;
        println!("The value of y in the inner scope is: {y}");
    }


    print!("The value of y is: {y}");
    
    let spaces = "string";
    let spaces = spaces.len();

    println!("{}", spaces);

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("{}", guess);

    let back_drop = 2.123;
    
    let asep_kargo: f32 = 3.12321; 

    println!("floating points f64 {}, and floating point f32 {}", back_drop, asep_kargo);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("sum is {}, difference is {}, product is {}, quotient is {}, truncated is {}, remainder is {}", sum, difference, product, quotient, truncated, remainder);

    let t = true;
    let f: bool = false; // with explicit type annotation.

    println!("{},{}", t, f);

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation

    println!("Char symbol {}, {}", c, z);
    let heart_eyed_cat = '😻' ;

    println!("Today emoji is {} ", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x is {}, y is {}, z is {}", x, y , z);

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("{}, {}, {}", five_hundred, six_point_four, one);    

    let a: [i32; 5] = [5, 4, 2, 1, 7];

    let b = [7; 10];

    let first = a[0];

    let second = a[1];

    let months = ["January", "February", "March", "April", "May", "June", "July", 
    "August", "September", "October", "November", "December"];
    
    println!("The first value on array a is {}", first);    

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

    println!("The value of the element at index {index} is: {element}");





}
