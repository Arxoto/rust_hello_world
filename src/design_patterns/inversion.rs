//! 反转  
//! 实现类似于控制反转的效果  
//! 一般用于框架 如 Bevy 或者 Axum  
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
pub trait FromContext {
    fn from_context(ctx: &Context) -> Self;
}

/// 再者定义一个抽象的代理函数  
/// 泛型 T 允许对于不同类型分别不同实现 起到类似于重载的作用
/// 需要框架侧多多个 Func 做实现
trait Handler<T> {
    fn call(self, ctx: &Context);
}

impl<T: FromContext, F: Fn(T)> Handler<T> for F {
    fn call(self, ctx: &Context) {
        self(T::from_context(ctx));
    }
}

impl<T1, T2, F> Handler<(T1, T2)> for F
where
    F: Fn(T1, T2),
    T1: FromContext,
    T2: FromContext,
{
    fn call(self, ctx: &Context) {
        self(T1::from_context(ctx), T2::from_context(ctx));
    }
}

/// 然后定义注册和触发方式
/// - 注册函数 由使用方将定制化的逻辑注入（这里没举例）
/// - 触发函数 由框架侧根据生命周期自动调用
#[inline]
fn trigger<T>(ctx: &Context, handler: impl Handler<T>) {
    handler.call(ctx);
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct Id(pub u32);

    impl FromContext for Id {
        fn from_context(ctx: &Context) -> Self {
            Id(ctx.id)
        }
    }

    /// 解构
    fn print_id(Id(id): Id) {
        println!("id is {}", id);
    }

    /// 对于复杂的对象 使用引用 标明生命周期
    pub struct Param(pub String);

    impl FromContext for Param {
        fn from_context(ctx: &Context) -> Self {
            Param(ctx.param.clone())
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
