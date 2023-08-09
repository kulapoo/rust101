#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }
}

fn main() {
    println!("{:?}", Rectangle::square(10));
}