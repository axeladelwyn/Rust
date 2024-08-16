fn main() {
    println!("Hello, world!");

    let user1 = build_user(
        String::from("boris322@gmail.com"),
        String::from("boris vladislav"),
        true,
        3,
    );

    println!(
        "User: {}, Email: {}, Active: {}, Sign-in count: {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let user2 = User {
        email: String::from("asepcobra69@gmail.com"),
        ..user1.clone()
    };

    println!(
        "User: {}, Email: {}, Active: {}, Sign-in count: {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    println!(
        "User: {}, Email: {}, Active: {}, Sign-in count: {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let black = Color(12, 12, 14);

    let origin = Point(-24, -34, 25);

    let subject = AlwaysEqual;

    black.display();

    origin.display();

}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Clone)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String, active: bool, sign_in_count: u64) -> User {
    User {
        active,
        username,
        email,
        sign_in_count,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

impl Color {
    // Method to display the color
    fn display(&self) {
        println!("Color ({}, {}, {})", self.0, self.1, self.2);
    }
}

impl Point {
    // Method to display the point
    fn display(&self) {
        println!("Point: ({}, {}, {})", self.0, self.1, self.2);
    }
}
struct AlwaysEqual;
