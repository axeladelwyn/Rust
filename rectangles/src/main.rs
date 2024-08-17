#[derive(Debug)]
// abstract structure
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation of area
// using data types of rectangle
// &self means self: &Self
// it also means rectangle:
// &Rectangle

// compare the width and the height of rectangle
// and declare true or false
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    // specific structure
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // print
    if rect1.width() {
        println!("The width of rectangle is not 0 it is, {}", rect1.width);
    } else {
        println!("The width value is 0 ");
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let sq = Rectangle::square(3);
    dbg! {sq};
}
