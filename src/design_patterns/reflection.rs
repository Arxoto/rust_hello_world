#![allow(dead_code)]

// 实现类似于控制反转的效果

struct Context {
    pub param: String,
    pub id: u32,
}

trait FromContext<'a> {
    fn from_context(ctx: &'a Context) -> Self;
}

/// 如果要支持引用的话 需要标记生命周期有传染性 否则需要clone
pub struct Param<'a>(pub &'a String);

impl<'a> FromContext<'a> for Param<'a> {
    fn from_context(ctx: &'a Context) -> Self {
        Param(&ctx.param)
    }
}

pub struct Id(pub u32);

impl FromContext<'_> for Id {
    fn from_context(ctx: &Context) -> Self {
        Id(ctx.id)
    }
}

trait Handler<'a, T> {
    fn call(self, ctx: &'a Context);
}

impl<'a, F, T> Handler<'a, T> for F
where
    F: Fn(T),
    T: FromContext<'a>,
{
    #[inline]
    fn call(self, ctx: &'a Context) {
        (self)(T::from_context(ctx));
    }
}

impl<'a, T1, T2, F> Handler<'a, (T1, T2)> for F
where
    F: Fn(T1, T2),
    T1: FromContext<'a>,
    T2: FromContext<'a>,
{
    #[inline]
    fn call(self, ctx: &'a Context) {
        (self)(T1::from_context(ctx), T2::from_context(ctx));
    }
}

/// 支持输入多种函数 其入参个数多样化
/// 但多样化需要手动对函数实现Handler
#[inline]
fn trigger<'a, T>(ctx: &'a Context, handler: impl Handler<'a, T>) {
    handler.call(ctx);
}

#[cfg(test)]
mod test {
    use super::*;

    /// 解构
    fn print_id(Id(id): Id) {
        println!("id is {}", id);
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
        trigger(&ctx, print_id);
        trigger(&ctx, print_param_id);
    }
}
