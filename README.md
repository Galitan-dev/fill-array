# Fill Array

## Usage
```rs
#[macro_use]
extern crate fill_array;

pub fn main() {
    fill![Vec::new(); 3]
}
```

## Output
```rs
#[macro_use]
extern crate fill_array;

pub fn main() {
    [Vec::new(), Vec::new(), Vec::new()]
}
```