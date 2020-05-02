    pub struct Sum {
        a: i32,
        b: i32,
    }

    impl Sum {
        pub fn new(a: i32, b: i32) -> Sum {
            Sum{ a: a, b: b}
        }

        pub fn sum(&self) -> i32 {
            self.a + self.b
        }
    }