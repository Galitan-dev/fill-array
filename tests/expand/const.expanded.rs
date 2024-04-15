#[macro_use]
extern crate fill_array;
const SIZE: usize = 5;
pub fn main() {
    {
        let mut array = [String::new(); SIZE];
        for i in 1..SIZE {
            array[i] = String::new();
        }
        array
    }
}
