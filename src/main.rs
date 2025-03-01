//! 这种包注释仅且必须在 包根(main.rs or lib.rs) 中使用，快使用 `cargo doc --open` 看一下效果吧！

#![allow(dead_code)]

mod course;
mod design_patterns;
mod project_structure;

use std::collections::HashMap;

use course::course1::test_all as test1;
use course::course2::test_all as test2;

/// 项目结构 和 基础课程(course)
fn run_test() {
    project_structure::inner_module::test();
    test1();
    test2();
}

/// VSCode 中调试 rust
/// 
/// 通用前提条件
/// - 设置里 搜索 "breakpoints" 勾选 "Allow setting breakpoints in any file."
/// - 打上断点
/// 
/// 使用 "rust-analyzer" 进行调试
/// - see https://code.visualstudio.com/docs/languages/rust#_debugging
/// - 执行命令调试
///   - "Rust Analyzer: Debug" (`Ctrl+Shift+P` || `Ctrl+P` && input ">")
///   - 或者在 main 方法上面找到 "Run|Debug" 点击运行
/// - 依赖插件 "rust-analyzer"
/// 
/// 使用 "C++(Windows)" 进行调试
/// - 使用 F5 开始调试
/// - 依赖插件 "C/C++ (ms-vscode.cpptools)"
/// - 可选 配置任务 终端-配置生成默认任务-"rust: cargo build" 否则每次修改需要手动 `cargo build`
/// - 编辑 .vscode/launch.json "configurations"
/// ```json
/// {
///     "name": "(Windows) Launch",
///     "type": "cppvsdbg",
///     "request": "launch",
///     // "preLaunchTask": "rust: cargo build",
///     "program": "${workspaceFolder}/target/debug/${workspaceFolderBasename}.exe",
///     "args": [],
///     "stopAtEntry": false,
///     "cwd": "${fileDirname}",
///     "environment": [],
///     "console": "integratedTerminal"
/// }
/// ```
/// 
/// 使用 "LLDB" 进行调试
/// - 使用 F5 开始调试
/// - 依赖插件 "CodeLLDB" （最近好像新出了个插件 "LLDB DAP" 可以研究下）
/// - vscode 左侧栏点击“运行和调试”，直接点击按钮“运行和调试”，选择 "LLDB" 会自动创建
/// - 实测不支持查看复杂变量的值 vec 和 hashmap （因此建议使用上面的方法）
///   - 安装 rustup 时有两个工具链可选 msvc(default) 和 gnu https://rust-lang.github.io/rustup/installation/windows.html
///   - 默认的 msvc 不包括 Rust-specific formatters for LLDB https://github.com/vadimcn/codelldb/wiki/Windows#debugging-rust-on-windows
#[allow(unused)]
fn run_debug() {
    let mut x = 5;
    x += 3; // 8

    let mut y = 42;
    let z = y / x; // 5
    let w = z * 3 - x; // 5 * 3 - 8 = 7

    println!("{}", w);

    let ll = vec![5, 6, 7];
    println!("{:?}", ll);

    let mm = HashMap::from([
        ("q", 2),
        ("w", 2),
        ("e", 4)
    ]);
    println!("{:#?}", mm);
}

/// 如何新建一个 rust 项目？使用命令 `cargo new project_name`
/// 开发时调试，编译并运行（不做优化） `cargo run`
/// 编译项目 `cargo build --release`
fn main() {
    println!("Hello, world!");

    // run_test();

    run_debug();
}
