pub struct Stepper{
    curr: i32, step: i32, max: i32,
}

impl Iterator for Stepper{
    type Item = i32;
    fn next(&mut self)->Option<i32> {
        if self.curr >= self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}
fn main() {

    let mut st = Stepper{curr:2, step:3, max: 15};
    loop{
        match st.next(){
            Some(v) => println!("loop {}", v),
            None => break,
        }
    }
    let mut n = 0;
    loop {
        n += 1;
        if n > 10{
            break;
        }
        println!("{}", n);
    }

    let mut st2 = Stepper{curr:3, step:4, max: 15,};

    while let Some(n) = st2.next(){
        println!("while {}", n);
    }
    let mut w = 0;
    while w < 10{
        w += 1;
        println!("{}", w);
    }

    for i in 0..10{
        println!("{}", i);
    }

    println!("Simple array example");
    let arr = [0, 1, 2, 3, 4, 5];
    for v in arr.iter(){
        println!("{}", v);
    }
    println!("All loops are done");
}
