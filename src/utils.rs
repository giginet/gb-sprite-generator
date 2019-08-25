use std::f64;

pub fn padded<T: Clone>(vec: &Vec<T>, len: usize, default_value: T) -> Vec<T> {
    let diff = len - vec.len();
    let mut new = vec.clone();
    if diff > 0 {
        for _ in 0..diff {
            new.push(default_value.clone());
        }
    }
    return new
}

pub fn next_multiple(multiple_of: u32, number: u32) -> u32 {
    let division = (number as f64 / multiple_of as f64);
    if division != division.trunc() {
        let target = division.ceil();
        return multiple_of * target as u32;
    }
    return number;
}
