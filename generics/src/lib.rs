pub mod my_lib {
    #[derive(Debug)]
    pub struct Point<T> {
        x: T,
        y: T
    }

    impl<T> Point<T> {
        pub fn x(&self) -> &T {
            &self.x
        }

        pub fn new(x: T, y: T) -> Self {
            Point {
                x,
                y
            }
        }
    }
}