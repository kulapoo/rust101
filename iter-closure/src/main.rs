

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();
    // let tae = v1.iter().collect::<Vec<i32>>();
    // println!("{:?}", tae);
    for it in v1_iter {
        println!("{it}");
    }
}
