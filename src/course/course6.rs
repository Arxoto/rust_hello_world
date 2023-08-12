#[cfg(test)]

/**
 * rust的异常处理
 */

mod error_handle {
    /// rust 的 panic 建议只能在必要的时候使用（一般推荐语法糖"?"）
    /// - golang 【推荐返回 (result, err) 】 panic 捕获异常 defer(run after return) + recover(catch Exception)
    /// - rust 【不建议捕获】 见 std::panic::catch_unwind() 有一些限制：传入的闭包要实现 UnwindSafe
    #[test]
    fn panic_error() {
        panic!("error occured");
        // println!("Hello, Rust"); // 中断 程序终止
    }

    #[test]
    fn result_error() {
        fn afunc(t: bool) -> Result<String, String> {
            if t {
                Ok("yes!".to_string())
            } else {
                Err("nono!".to_string())
            }
        }

        // 最基础版本
        println!("use match: false");
        let a = afunc(false);
        match a {
            Ok(info) => {
                println!("ok: {info}");
            }
            Err(error_info) => {
                println!("err: {}", error_info);
            }
        }

        // 或者使用语法糖
        println!("use if-let: true");
        let a = afunc(true);
        if let Ok(info) = a {
            println!("ok: {info}");
        }

        // or this 这种处理方式比较像golang
        // 也可以用 unwrap_or_else + lambda (+ panic!) 实现
        println!("use is_err(): false");
        let a = afunc(false);
        if a.is_err() {
            println!("err: {}", a.unwrap_err());
        } else {
            println!("ok: {}", a.unwrap());
            // a.expect("类型错误"); // 自定义报错信息
        }

        // or this 另一种方式 ?必须用在返回值 `Result` or `Option` 的函数内部
        fn inner() -> Result<String, String> {
            let a_info = afunc(false)?; // 异常相当直接 return E
            return Ok(a_info);
        }
        println!("use ?: {}", inner().unwrap_or("result is Err, default error".to_string()));
    }
}
