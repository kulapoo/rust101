#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let mut list = vec![
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    let mut sort_count = 0;
    let val = String::from("val val");
    // let mut sort_ops = Vec::new();


    list.sort_by_key(|r| {
        sort_count += 1;
        // sort_ops.push(val); this will fail -> cannot move "val" as this iter is using FnMut
        r.width
    });

    println!("rect list {:#?}", list);
    println!("sort count {}", sort_count);
}
