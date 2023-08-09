// fn main() {
//     let some_option_value: Option<i32> = Some(5);
//     if let Some(x) = some_option_value {
//         println!("{x}");
//     }

//     if let x = 5 {
//         println!("{}", x);
//     };

//     let num = Some(4);

//     match num {
//         Some(x) if x % 2 == 0 => println!("num is even {x}"),
//         Some(x) => println!("num is even {x}"),
//         None => ()
//     }
// }

// match guard
// fn main () {
//     let x = Some(5);
//     let y = 10;

//     match x {
//         Some(50) => println!("Got 50"),
//         Some(n) if n == y => println!("Matched, n = {n}"),
//         _ => println!("Default case, x = {:?}", x),
//     }
//     println!("at the end: x = {:?}, y = {y}", x);
// }

enum Message {
    Hello { id: i32 }
}


fn main () {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7
        } => {
            println!("Found an id in range: {}", id_variable);
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}