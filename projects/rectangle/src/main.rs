#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}
fn main() {
    let rect = Rectangle {
        height: 30,
        width: 50,
    };

    println!("{:#?}", rect);
    println!("The area of the rectangle is: {}", area(&rect));
}

fn area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
