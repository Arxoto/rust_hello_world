//! 反转  
//! 实现类似于控制反转的效果  
//! 但是好像没啥用 反正还是需要直接读取上下文类型里的内容  
//!
//! 这里增加对 静态分发 和 动态分发 的说明
//! - 静态分发（泛型） 编译期单态化 存在类型膨胀（编译时间和二进制文件）及缓存失效概率增加等问题
//!   - 泛型语法 `fn foo<T: Trait>(arg: T) {}` 可以使用 turbo-fish 语法 `foo::<usize>(1)`
//!   - 简短语法 `fn foo(arg: impl Trait) {}` 不支持 turbo-fish 语法
//! - 动态分发（类型擦除的对象指针和虚函数表） 运行时动态生成胖指针 解引用导致开销大
//!   - 一般跟引用或指针一起使用 `fn foo(arg: &dyn Trait) {}` or `fn foo() -> Box<dyn Trait> {}`
//! 
//! 这里增加对 泛型 和 关联类型 的说明
//! - 实现带泛型的特征时 对于同一类特征可以实现多次（所以称之为泛型）
//! - 实现有关联类型的特征时 仅能存在一个关联类型（因为关联类型与特征相关联）

#![allow(dead_code)]

/// 首先定义一个上下文类型存放所有东西
pub struct Context {
    pub id: u32,
    pub param: String,
}

/// 其次定义一个抽象特征 支持从上下文中进行类型转换  
/// 这里有无损转换的需求 因此用到生命周期注释  
/// 这个特征可以由 使用方 根据需求自由实现
pub trait FromContext<'a> {
    fn from_context(ctx: &'a Context) -> Self;
}

/// 再者定义一个抽象的代理函数  
/// 这里的的生命周期和泛型声明其实一开始不需要 是在做多态实现的时候带进来的约束
///
/// 接着精彩的来了 为多个参数的 Func 实现该特征 完成函数与函数之间的转换  
/// 这一步需要由框架测来实现
trait Handler<'a, T> {
    fn call(self, ctx: &'a Context);
}

impl<'a, T: FromContext<'a>, F: Fn(T)> Handler<'a, T> for F {
    fn call(self, ctx: &'a Context) {
        self(T::from_context(ctx));
    }
}

impl<'a, T1, T2, F> Handler<'a, (T1, T2)> for F
where
    F: Fn(T1, T2),
    T1: FromContext<'a>,
    T2: FromContext<'a>,
{
    fn call(self, ctx: &'a Context) {
        self(T1::from_context(ctx), T2::from_context(ctx));
    }
}

/// 然后定义注册和触发方式
/// - 注册函数 由使用方将定制化的逻辑注入（这里没举例）
/// - 触发函数 由框架侧根据生命周期自动调用
#[inline]
fn trigger<'a, T>(ctx: &'a Context, handler: impl Handler<'a, T>) {
    handler.call(ctx);
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct Id(pub u32);

    impl FromContext<'_> for Id {
        fn from_context(ctx: &Context) -> Self {
            Id(ctx.id)
        }
    }

    /// 解构
    fn print_id(Id(id): Id) {
        println!("id is {}", id);
    }

    /// 对于复杂的对象 使用引用 标明生命周期
    pub struct Param<'a>(pub &'a String);

    impl<'a> FromContext<'a> for Param<'a> {
        fn from_context(ctx: &'a Context) -> Self {
            Param(&ctx.param)
        }
    }

    fn print_param_id(Param(param): Param, Id(id): Id) {
        println!("param is {}, id is {}", param, id);
    }

    #[test]
    fn test() {
        let ctx = Context {
            param: "asd".into(),
            id: 123,
        };

        // 演示
        trigger(&ctx, print_id);
        trigger(&ctx, print_param_id);
    }
}
