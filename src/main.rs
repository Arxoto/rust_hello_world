/*!
 * 这种包注释仅且必须在 包根(main.rs or lib.rs) 中使用，快使用 `cargo doc --open` 看一下效果吧！
 */

mod project_structure;
mod course;
mod design_patterns;

use course::course1::test_all as test1;
use course::course2::test_all as test2;

fn main() {
    project_structure::inner_module::test();
    println!("Hello, world!");
    test1();
    test2();
}