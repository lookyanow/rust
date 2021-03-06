#[derive(Debug)]
pub struct Linkedlist<T>{
    data:T, 
    next: Option<Box<Linkedlist<T>>>,
}

impl <T:std::ops::AddAssign> Linkedlist<T> {
    pub fn add_up(&mut self, n: T){
        self.data += n;
    }
}
fn main() {
    let mut ll = Linkedlist{
        data: 3, 
        next: Some(Box::new(Linkedlist{
            data: 2, 
            next: None,
        })),
    };

    if let Some(ref mut v) = ll.next{
        v.add_up(10);
    }
    println!("List: {:?}", ll);

    let mut v = Vec::new();
    v.push("hello".to_string());
    v.push("world".to_string());

    for i in 0..15{
        v.push(i.to_string());
    }

    println!("vector of strings {:?}", v);
}
