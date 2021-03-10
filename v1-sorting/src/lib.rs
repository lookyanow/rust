use std::fmt::Debug;
// O(n^2)
pub fn bubble_sort<T: PartialOrd + Debug> (v: &mut [T]) {
    for _ in 0..v.len() {
        println!("{:?}", v[0]);
        for i in 0..v.len() - 1 {
            if v[i] > v[i+1]{
                v.swap(i, i+1);
            }
        }
    }
}

// pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
//     if v.len() <= 1 {
//         return v;
//     }
//     let b = v.split_off(v.len() / 2);
//     let a = merge_sort(v);
//     b = merge_sort(b);
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bubble_sort() {
        let mut v = vec![4,6,1,8,11,13,3];
        bubble_sort(&mut v);
        assert_eq!(v, vec![1,3,4,6,8,11,13]);
    }
}

