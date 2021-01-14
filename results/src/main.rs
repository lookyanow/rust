fn main() {
    println!("Hello, world!");
    let a = divide(10, 5);
    // let b = divide(10, 0);

    if let Ok(v) = a{
        println!("vaulue of a = {}", v);
    }
}

fn divide(a:i32, b:i32) -> Result<i32, String>{
    if b == 0{
        return Err("Connot divide by zero".to_string()); 
    }
    Ok(a / b)
}