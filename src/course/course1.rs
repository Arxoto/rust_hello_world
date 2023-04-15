/// 基础语法 变量 语句
pub fn test_all() {
    println!("==========");
    test_shadowing();
    println!("==========");
    test_tuple();
    println!("==========");
    test_func();
    println!("==========");
    test_if();
    println!("==========");
    test_while_for_loop();
}

/// # 文档注释
/// 探究 常量 不可变变量 可变变量 重影机制
fn test_shadowing() {
    // 不可变变量可以多次定义 - 重影Shadowing 地址会改变

    let a = 1;
    println!("Hello, world! tmp is {}.", a);
    let addr = &a as *const i32 as usize;
    println!("addr: 0x{:X}", addr);

    let a = 2;
    println!("Hello, world! tmp is {}.", a);
    let addr = &a as *const i32 as usize;
    println!("addr: 0x{:X}", addr);

    // 这是可变变量 允许直接修改
    // 个人理解：只有重新赋值的情况才需要mut 然后他具有传染性 只要struct里的任何一个属性需要修改 整个struct就需要是mut的

    let mut a = 2;
    println!("Hello, world! tmp is {}.", a);
    let addr = &a as *const i32 as usize;
    println!("addr: 0x{:X}", addr);

    a = 2;
    println!("Hello, world! tmp is {}.", a);
    let addr = &a as *const i32 as usize;
    println!("addr: 0x{:X}", addr);

    // 而常量 不能重新定义

    const B_TMP: i64 = 0;
    // const B_TMP: i64 = 1; // 会报错
    println!("Hello, world! tmp is {}.", B_TMP);
}

/// 元组简单使用
fn test_tuple() {
    // 元组 tuple

    let tup = (1, true, 6.1);
    let (x, y, z) = tup;
    println!("{}:{}:{}", x, y, z);
}

/// 函数体表达式 函数体 lambda
fn test_func() {
    // 函数体表达式 不可用return
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("x is {}", x);
    println!("y is {}", y);

    // 函数 返回类型必须明确 不可重复定义

    fn five() -> i32 {
        5
    }
    println!("five() 's value is {}", five());

    fn six() -> i32 {
        return 6;
    }
    println!("six() 's value is {}", six());

    // lambda

    let seven = || 7;
    println!("seven() 's value is {}", seven());
    let add = |a, b| a + b;
    println!("add(six(), seven()) 's value is {}", add(six(), seven()));
}

/// 条件语句
fn test_if() {
    // if条件可以加括号也可以不加 if语句能返回值
    let a = 1;
    let b = if a == 0 { false } else { true };
    println!("a is {}, b is {}", a, b);
}

/// 循环语句
fn test_while_for_loop() {
    // 目前没有 do-while 和 fori
    println!("while");
    let mut i = 0;
    while i < 3 {
        println!("i is {}", i);
        i += 1;
    }

    // 自带的 while-true 及 loop  这里用了返回值的方式
    println!("loop");
    let s = ['R', 'U', 'S', 'T'];
    let mut i = 0;
    let index = loop {
        let ch = s[i];
        if ch == 'S' {
            break i;
        }
        i += 1;
    };
    println!("index of 'S' is {}", index);

    // for 用于遍历迭代
    println!("for in iter");
    let a = [0,1,2];
    for i in a.iter() {
        println!("i is {}", i);
    }
    println!("for in 0..5");
    for i in 0..5 {
        println!("i is {}", i);
    }
}
