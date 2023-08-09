
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 20,
        height: 25
    };

    println!("Area of triangle {}", area(&rect));
    println!("Rect {:#?}", rect);
    println!("Rect {:#?}", &rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}