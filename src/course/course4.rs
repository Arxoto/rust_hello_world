//! 本章学习：
//! 给函数标注声明周期（编译器无法推导生命周期的情况下，手动标注）

#[cfg(test)]
mod test_lifetime {

    /// 生命周期
    /// - 生命周期lifetime 是针对于引用reference 的
    /// - 为了能够进行借用规则检查 编译器需要知道所有引用的生命周期
    /// - 部分情况编译器无法推导生命周期 需要手动标注
    #[test]
    fn lifetime() {
        /// 生命周期
        /// - 不手动标注：编译器无法推断返回值的生命周期（要求在函数签名中显示写明）
        /// - 手动标注：要求编译器检查确保 返回值的生命周期与参数生命周期中的较小值一致
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("abcd");
        let string2 = String::from("xyz");

        let result = longest(&string1, &string2);
        println!("The longest string is {}", result);

        // 下面的会因生命周期报错
        // let string1 = String::from("abcd");
        // let result;
        // {
        //     let string2 = String::from("xyz");
        //     result = longest(&string1, &string2);
        // }
        // println!("The longest string is {}", result);
    }

}
