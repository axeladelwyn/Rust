fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a string

    {
        let s = String::from("hello");
        println!(" the value of s is {s}");
    }
    println!("{s}");

    let x = 7;
    let y = x;





}
