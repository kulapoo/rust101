use std::{rc::Rc};
use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil
}

impl List {
    fn tail(&self) -> Option<&Rc<List>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(
        Cons(5, Rc::new(
            Cons(10, Rc::new(Nil)
        ))
    ));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
