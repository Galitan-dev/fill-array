#[macro_use]
extern crate fill_array;

const SIZE: usize = 5;

pub fn main() {
    fill![String::new(); SIZE]
}
