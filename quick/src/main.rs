

fn main() {
    let mut s = String::from("hello");
    // let r1 = &mut s; // no problem
    // let r2 = &s; // no problem
    // println!("{}", r1)
    {
        let r1 = &mut s;
        change(r1);
    }
}

fn change(some_string: &mut String) {
    let some_string = String::from("p;ota");
    println!("{}", some_string)
}