#[derive(Debug)]
pub struct Person{
    name: String, age: i32, children: i32, fave_color: Color
}

impl Person{
    pub fn print(self) -> String {
        format!("name = {}, age = {} has {} children", 
        self.name, self.age, self.children)
    }
}

#[derive(Debug)]
pub enum Color{
    Red,
    Green,
    Blue,
}

fn main() {
    let p = Person{
        name: "Ivan".to_string(),
        age: 35, 
        children: 2,
        fave_color: Color::Green,
    };
    // println!("Get person struct{}", p.print());
    println!("Get person struct{:?}", p);

    let c = Color::Red;
    println!("{:?}", c)
}
