use std::fs::File;
use std::io;
use std::io::Read;


fn read_data_from_file() -> Result<String, io::Error>{
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_data_file(filename: &str) -> Result<String, io::Error>{
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {

    match read_data_from_file(){
        Ok(s) => println!("{}", s),
        Err(e) => panic!(e),
    }

    let filename = "hello.txt";
    match read_data_file(filename){
        Ok(s) => println!("{}", s),
        Err(e) => panic!(e),
    }
}
