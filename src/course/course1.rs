#![allow(dead_code)] // 防止提示 fields xxx are never read

/**
 * 本章学习：
 * 普通注释（本注释内容） 文档注释（解析成HTML帮助文档）
 * 格式化和输出
 * 不可变变量 可变变量 常量
 * 原生类型：元组（用作返回值时很有用）
 * 原生类型：数组（类型大小确定）和与其强相关的切片
 * 自定义类型：结构体
 * 自定义类型：枚举
 * 别名（见枚举的测试函数中的说明）
 * 函数 函数体表达式 lambda
 * 条件 循环
 */

/// 基础语法 变量 语句
pub fn test_all() {
    println!("\n\ncourse 1:");
    println!("==========");
    test_format_print();
    println!("==========");
    test_shadowing();
    println!("==========");
    test_tuple();
    println!("==========");
    test_array_slice();
    println!("==========");
    test_struct();
    println!("==========");
    test_enum();
    println!("==========");
    test_func();
    println!("==========");
    test_if();
    println!("==========");
    test_while_for_loop();
}

/// 格式化输出
/// - 实现了 fmt::Display 特征（trait）后该结构体可以被打印
/// - 对于泛型容器（generic container）（如 Vec<T>）没实现该特征 需要使用 fmt::Debug
fn test_format_print() {
    let ss = format!(
        "a is {0}, b is {b_name}, num={num:>0width$}\n",
        "aaa",
        b_name = "bname",
        num = 3,
        width = 5
    );
    print!("{ss}");

    println!("normal print"); // io::stdout
    eprintln!("error print"); // io::stderr

    #[derive(Debug)]
    struct Structure1(i32);
    #[derive(Debug)]
    struct Structure2 {
        id: i32,
        name: String,
    }
    println!(
        "print by add 'derive(Debug)': {:?} {1:?}\n{1:#?}",
        Structure1(3),
        Structure2 {
            id: 1,
            name: String::from("aa")
        }
    );
}

/// # 文档注释
/// 探究 常量 不可变变量 可变变量 变量遮蔽（variable shadowing）（也有翻译重影机制）
fn test_shadowing() {
    // 不可变变量可以多次定义 - 遮蔽Shadowing 地址会改变

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
    // 另一种常量 用 static 声明。具有 'static 生命周期的，可以是可变的变量（译注：须使用 static mut 关键字）
}

/// 元组简单使用
fn test_tuple() {
    // 元组 tuple
    let tup = (1, true, 6.1);
    // 解构（deconstruct）
    let (x, y, z) = tup;
    println!("{}:{}:{}", x, y, z);
}

/// 数组和切片
fn test_array_slice() {
    // 定长数组 array
    let xs = [1, 2, 3, 4, 5];
    // 所有元素可以初始化成相同的值
    let ys = [0; 9];

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", std::mem::size_of_val(&xs));

    /*
    数组（array）是一组拥有相同类型 T 的对象的集合，在内存中是连续存储的。
    数组使用中括号 [] 来创建，且它们的大小在编译时会被确定。
    数组的类型标记为 [T; length]（译注：T 为元素类型，length 表示数组大小）。

    切片（slice）类型和数组类似，但其大小在编译时是不确定的。
    相反，切片是一个双字对象（two-word object），第一个字是一个指向数据的指针，第二个字是切片的长度。
    这个 “字” 的宽度和 usize 相同，由处理器架构决定，比如在 x86-64 平台上就是 64 位。
    slice 可以用来借用数组的一部分。
    slice 的类型标记为 &[T]。
    */

    // 此函数借用一个 slice
    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);
}

/// 结构体
fn test_struct() {
    // 单元结构体 据说在泛型中很有用
    struct Unit;

    // 元组结构体
    struct Pair(i32, f32);
    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);
    // 访问元组结构体的字段
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    // 经典的 C 语言风格结构体（C struct）
    struct Point {
        x: f32,
        y: f32,
    }
    // 实例化结构体 `Point`  带后缀的字面量 结构体初始化
    let x = 10.3f32;
    let y = 0.4f32;
    let point: Point = Point { x, y };
    // 访问 point 的字段
    println!("point coordinates: ({}, {})", point.x, point.y);
    // 使用结构体更新语法创建新的 point，这样可以用到之前的 point 的字段
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` 与 `point.y` 一样，因为这个字段就是从 `point` 中来的
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    // 使用 `let` 绑定来解构 point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;
    println!("second point: ({}, {})", left_edge, top_edge);
}

/// 枚举
fn test_enum() {
    // 枚举类型的取值独立 也可以给予赋值
    enum Event {
        A,
        B(char),
        C(String),
        D {x: i64, y: i64},
    }

    fn inspect(event: Event) {
        match event {
            Event::A => println!("the a"),
            Event::B(c) => println!("the char is {c}"),
            Event::C(s) => println!("the string is {s}"),
            Event::D { x, y } => println!("the x,y is {x},{y}"),
        }
    }

    // 别名 主要用途是避免写出冗长的模板化代码（boilerplate code）。如 IoResult<T> 是 Result<T, IoError> 类型的别名。
    type E = Event;

    inspect(E::A);
    inspect(E::B('m'));
    inspect(E::C(String::from("ssss")));
    inspect(E::D { x: 2, y: 3 });
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
    let a = [0, 1, 2];
    for i in a.iter() {
        println!("i is {}", i);
    }
    println!("for in 0..5");
    for i in 0..5 {
        println!("i is {}", i);
    }
}
