use std::fmt::Write;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main () {
    let config = Message::Write(String::from("tae"));

    match config {
        Message::Quit => {
            println!("quit")
        }
        Message::Write(s) => {
            println!("{}", s);
        }
        _ => println!("wala")
    }
}

// fn main() {
//     let five = Some(5);
//     let six = plus_one(five);
//     let none = plus_one(None);

//     println!("{}", six.unwrap_or_else(|| 5));
// }


// fn main() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x + y.unwrap();


//     println!("{}", sum);
// }