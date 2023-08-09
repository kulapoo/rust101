trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Fly pilot human")
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Fly wizard human")
    }
}

impl Human {
    fn fly(&self) {
        println!("Im a human who can fly")
    }
}

fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
