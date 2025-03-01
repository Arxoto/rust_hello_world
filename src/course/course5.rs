//! rust 实现简单链表

#![allow(dead_code)]

use std::mem;

/// rust linked list
pub struct List {
    head: Link,
}

/// 为什么不直接用Node：栈长度无法在编译期确定
/// 为什么不用引用：需要提供给外界使用 引用失效
/// 所以可以这么认为：Option就是null Box就是指针
type Link = Option<Box<Node>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        // let new_node = Box::new(Node {
        //     elem: elem,
        //     next: self.head, // 这里报错 可以用clone方法解决 但是会导致整个链表的复制
        // });
        // // 如果是其他语言 在这个位置会有两个指向Link的指针 Rust不允许这种情况发生
        // self.head = Some(new_node);

        let new_node = Box::new(Node {
            elem,
            next: std::mem::replace(&mut self.head, None), // 这里进行了原子操作 就没有上面的问题了 可以优化为self.head.take()
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // 同理这里也可以优化
        // self.head.take().map(|node| {
        //     self.head = node.next;
        //     node.elem
        // })
        match std::mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    /// 因为涉及到Box<> 他会在self.ptr.drop(); 后调用deallocate(self.ptr); 因此不是尾递归可能会爆栈
    ///
    /// 大概是这个意思 所以需要手动实现drop方法
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}