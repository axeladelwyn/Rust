fn main() {

    let x: i8 = 5;
    let y = Some(5);
    let z = Some(8);
    let sum = match y {
        Some(value) => x + value,
        None => x,
    };
    let sum = match z {
        Some(value) => match y {
            Some(y_value) => Some(y_value + value),
            None => None,
        },
        None => None
    };
    println!("Sum: {:?}", sum);

    let mut vec: Vec<usize> = Vec::new();
    vec.push(10);
    vec.push(20);
    vec.push(30);

    for value in &vec {
        println!("{}", value);
    }
        
}
enum Option<T> {
    None,
    Some(T),
}

