#[macro_use]
extern crate fill_array;
const SIZE: usize = 5;
pub fn main() {
    core::array::from_fn::<_, SIZE, _>(|_| Vec::new())
}
