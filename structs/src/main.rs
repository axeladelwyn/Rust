fn main() {
    println!("Hello, world!");

    let mut  user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 8,
    };

    user1.email = String::from("anotherermail@example.com");

    let rect  = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} sqquare pixels.", rect.area());


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



struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}