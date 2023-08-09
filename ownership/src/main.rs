fn main() {
    let mut s = String::from("Hello");

    change(&mut s);

    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str("tae tae");
}