mod project_structure;
mod course;

use course::course1::test_all as test1;
use course::course2::test_all as test2;

fn main() {
    project_structure::inner_module::test();
    println!("Hello, world!");
    test1();
    test2();
}