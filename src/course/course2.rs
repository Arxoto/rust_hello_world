//! 本章学习：
//! - 基本类型的赋值是 值拷贝（栈）
//! - 其他类型的赋值伴随着 所有权转移（不标记为Copy）
//! - 所有权是唯一的
//! - 借用 / 引用（let ref xx = 1;  xx is &i32 目前表现和借用一致 这里不做区分）

/// 基本类型数据 是存在栈中的 这时的赋值是值拷贝
fn ownership_stack_basic_types() {
    let a = 1;
    println!("a addr: 0x{:X}", &a as *const i32 as usize);
    let b = a;
    println!("b addr: 0x{:X}", &b as *const i32 as usize);
    println!(
        "basic types in stack -> value copy when assignment: a is {}, b is {}",
        a, b
    );
}

/// 自定义类型 默认情况下也是存在栈中的  
/// 但为了内存安全 实际值唯一存在（所有权限制）  
/// 对于某些不包含指针的类型（意味着能安全地值拷贝多份） 需要手动标记 `#[derive(Clone, Copy)]`  
fn ownership_stack_non_basic_types() {
    struct Data(i32);
    fn print_data(data: Data) {
        println!(
            "inner func: addr: 0x{:X}, data: ({})",
            &data as *const Data as usize, data.0
        );
    }

    let d1 = Data(1);
    println!("origin addr: 0x{:X}", &d1 as *const Data as usize);
    let d2 = d1;
    println!("new addr: 0x{:X}", &d2 as *const Data as usize);
    // d1 不可用
    println!(
        "non-basic types in stack -> value copy and ownership moved when assignment to a new value"
    );

    print_data(d2);
    println!("non-basic types in stack -> value copy and ownership moved when use func");
    // d2 不可用

    // ========== 自定义类型实现 Copy 特征
    #[derive(Clone, Copy)]
    struct DataCopyable(i32);

    let d1 = DataCopyable(0);
    let d2 = d1;
    println!("d1({}), d2({})", d1.0, d2.0);
    println!("non-basic types in stack -> value copy and ownership not moved when assignment impl trait Copy");
    println!("non-basic types in stack -> trait Copy means can be deep-copied directly");
}

/// 存在堆中的数据  
/// 栈中为地址 赋值时 栈上规则相同 发生值拷贝并且所有权转移（这里的值为指针地址偏移量）  
/// 堆中为实际变量值 不动  
fn ownership_heap() {
    struct Data(i32);
    let a = Box::new(Data(0));
    // 发现一种简单的打印指针的方式
    println!("a heap addr: {:p}", a.as_ref());
    let b = a;
    println!("b heap addr: {:p}", b.as_ref());
    // a 不可用
    println!("non-basic types in heap -> addr is a value in stack and value in heap, {}", b.0);
}

/// 复制值时可以克隆 一般认为消耗比借用和所有权转移（直接值拷贝）大
fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("clone: {} & {}", s1, s2);
}

/// 函数入参时注意所有权的转移 同样的返回值也会转移  
/// 注：print 是个宏 宏展开后是引用传递的
fn ownership_func() {
    fn takes_ownership(some_string: String) {
        println!("takes_ownership \"{}\"", some_string);
        // print 不会获取所有权 所以可以在后面继续使用 但这里会发生循环所以注释掉了
        // takes_ownership(some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("makes_copy {}", some_integer);
    }

    let s = String::from("i am 's'");
    takes_ownership(s);
    println!("s is dead");
    // println!("s is dead {}", s);

    let x = 5;
    makes_copy(x);
    println!("x is still alive {}", x);

    println!("basic types and struct-impl-Copy-trait not move ownership when use func");
}

/// 借用获得引用
/// - 引用不会获得值的所有权。
/// - 引用只能租借（Borrow）值的所有权。
/// - 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权（可以认为指向栈的地址）
fn ownership_reference() {
    fn m_print(s: &String) {
        println!("the string is {}", s);
    }
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);
    m_print(s2);
    // 引用本身的传递不会发生所有权改变 s2还可以继续使用
    println!("ref not take ownership: s1 is {}, s2 is {}", s1, s2);

    let s3 = s1; // 所有权改变
    let s2 = &s3; // 之前租借的所有权失效了 需要重新租借
    println!("s1 ownership move to s3, so s2 need borrow again: s2 is {}, s3 is {}", s2, s3);

    // 可变变量的引用 可以修改值 但不能多重引用 称为【独占】
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s1_mut_ref = &mut s1; // 演示独占 注意位置在借用和修改中间 在更前面或者更后面都不会报错
    s2.push_str("world");
    println!("mut ref must be only one: {}", s1);
}

/// 所有权是 rust 特有的设计
/// - 每一个值（堆中的）都有一个被称为其 所有者（owner）的变量。
/// - 值有且只有一个所有者。
/// - 当所有者（变量）离开作用域，这个值将被丢弃。
pub fn test_all() {
    println!("\n\ncourse 2:");
    let test_func = [
        ownership_stack_basic_types,
        ownership_stack_non_basic_types,
        ownership_heap,
        clone,
        ownership_func,
        ownership_reference,
    ];
    for func in test_func.iter() {
        println!("==========");
        func();
    }
}
