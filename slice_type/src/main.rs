fn main() {

    let s = String::from("hello world");

    let _b = "hello, world!";

    let my_string = String::from("hellowwwww naana?");

    let _word = first_word(&my_string[0..6]);

    let _word = first_word(&my_string[..]);

    let word = first_word(&my_string[0..5]);

    println!("{word}");

    let my_string_literal = "hello world";

    let word = first_word(&my_string_literal[..]);
    
    println!("{word}");

    let first = first_word(&s);

    println!("{first}");

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for ( i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
    
}

