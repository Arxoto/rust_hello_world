/// 基本类型数据 是存在栈中的 这时的赋值是值拷贝
fn ownership_stack() {
    let a = 1;
    let b = a;
    println!("a is {}, b is {}", a, b);
}

/// 存在堆中的数据 赋值时拷贝地址 所有权转移
fn ownership_heap() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2); // s1 不可用
}

/// 复制值时可以克隆 但消耗较大
fn clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} & {}", s1, s2); // s1 不可用
}

/// 函数入参时注意所有权的转移 同样的返回值也会转移
/// 注：println是个宏 比较特殊
fn ownership_func() {
    fn takes_ownership(some_string: String) {
        println!("takes_ownership {}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("makes_copy {}", some_integer);
    }
    
    let s = String::from("hello");
    takes_ownership(s);
    // 参数传入时 地址拷贝 所有权转移 这里的s已经失效

    let x = 5;
    makes_copy(x);
    // 参数传入时 值拷贝 x仍然有效
    println!("x is still alive {}", x);
}

/// 引用
/// - 引用不会获得值的所有权。
/// - 引用只能租借（Borrow）值的所有权。
/// - 引用本身也是一个类型并具有一个值，这个值记录的是别的值所在的位置，但引用不具有所指值的所有权（可以认为指向栈的地址）
fn ownership_reference() {
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);
    // 之前租借的所有权失效了 需要重新租借
    let s3 = s1;
    let s2 = &s3;
    println!("s2 is {}, s3 is {}", s2, s3);

    // 可变变量的引用 可以修改值 但不能多重引用（多线程安全考虑？
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    s2.push_str("world");
    println!("{}", s1);
}

/// 所有权是 rust 特有的设计
/// - 每一个值都有一个被称为其 所有者（owner）的变量。
/// - 值有且只有一个所有者。
/// - 当所有者（变量）离开作用域，这个值将被丢弃。
pub fn test_all() {
    let test_func = [ownership_stack, ownership_heap, clone, ownership_func, ownership_reference];
    for func in test_func.iter() {
        println!("==========");
        func();
    }
}