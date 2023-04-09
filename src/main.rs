mod project_structure;
mod course;

use course::course1::test_all;

fn main() {
    project_structure::inner_module::test();
    println!("Hello, world!");
    test_all();
}