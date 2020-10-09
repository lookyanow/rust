

fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum+=value;
    }
    sum
}

fn main() {
    let values = vec![1, 2, 3, 4, 5];
    // let sum = take_ownership_sum(values);
    let sum = borrow_sum(&values);
    println!("{}, length: {}" , sum, values.len());
}
