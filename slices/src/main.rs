fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.height
    }
}


fn main() {
    let s = String::from("hello world");

    let word = first_word(&s);

    println!("first word is: {}", word);

    let rec = Rectangle{ width: 30, height: 50 };
    println!("{}", rec.area());

}
