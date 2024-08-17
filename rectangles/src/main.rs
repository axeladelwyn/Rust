fn main() {
    // Using struct as the structure 
    // and put the value in
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 80,
    };

    // displaying the area of rectangle
    dbg!(&rect1);

    
    
}
// function to calculate rectangle area
fn area(rectangle: &Rectangle) -> u32 {
    // formula to calculate rectangle area
    rectangle.width * rectangle.height  
}

// struct creation for the width
// and height of rectangle
// add a debug so you can print 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}