fn main() {
    let s1 = String::from("hello");

    let s = &s1;

    let len = calculate_length(s);

    println!("The length of '{}' is {}.", s, len);

    let mut str = String::from("hello");

    change(&mut str);

    println!("str: {}", str);

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}