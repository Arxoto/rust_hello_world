pub mod inner_module {
    /// 包Package: 及一个工程 必须由一个 Cargo.toml 文件来管理
    ///
    ///
    /// 箱Crate: 包(Package)包含箱(Crates)
    /// 1. 只能包含0或1个类库箱(library crates)
    /// 1. 可以包含任意多个二进制箱(binary crates)
    /// 1. 至少有一个箱(Crate), 可以是类库箱(library crates), 也可以是二进制箱(binary crates)
    /// 
    /// 
    /// - 默认, 一个箱(crate):
    ///   - cargo new my-project > src/main.rs 是二进制箱(binary crate)的根文件, 该箱(crate)与包(package)同名
    ///   - cargo new --lib my-lib > src/lib.rs 是类库箱(library crate)的根文件, 该箱(crate)与包(package)同名
    /// - 多个二进制箱(binary crates): 
    ///   - 在src/bin目录下创建.rs文件, 每个文件对应一个二进制箱(binary crate).
    ///
    ///
    /// 模块Module: 组成工程的组织模块 可以是
    /// - 文件 一个rs文件默认为一个模块
    /// - 文件夹 使用mod.rs
    /// - mod关键字定义 如这里的inner_module 可以多层嵌套
    pub fn test() {
        println!("project_structure")
    }
}
