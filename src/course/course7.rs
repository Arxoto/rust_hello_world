#[cfg(test)]
mod test_trait {

    /// 特性
    /// - 类似于 Java 里的接口 Interface
    /// - 可以有默认实现
    trait Runner {
        fn run(&self);
        fn run_default(&self) {
            println!("run_default");
        }
    }

    /// 特性做入参
    /// - 语法糖 相当于函数使用泛型
    fn run_something(runner: &impl Runner) {
        runner.run();
    }

    /// 自定义的结构体 用了泛型
    struct Aser<T> {
        inner: T,
    }

    impl<T> Aser<T> {
        /// 给结构体实现方法
        fn new(inner: T) -> Self {
            Aser { inner }
        }
    }

    impl<T: std::fmt::Display> Runner for Aser<T> {
        /// 给结构体实现特性
        /// - 可以多个特性相加表示同时实现多个
        /// - 这里是一个语法糖 相当于 where
        /// ```
        /// impl<T> Runner for Aser<T>
        /// where
        ///     T: std::fmt::Display,
        /// {
        ///     fn run(&self) {
        ///         println!("{}", self.inner);
        ///     }
        /// }
        /// ```
        fn run(&self) {
            println!("{}", self.inner);
        }
    }

    #[test]
    fn test_once() {
        let a = Aser::new("inner info".to_string());
        run_something(&a);
        a.run_default();
    }
}
