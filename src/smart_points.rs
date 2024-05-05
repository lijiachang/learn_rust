/*
# 指针：一个变量在内存中包含的是一个地址（指向其他数据）
Rust中最常见的指针就是“引用”
引用：
* 使用&
* 借用它指向的值
* 没有其余开销

# 智能指针
* 行为和指针相似
* 有额外的元数据和功能

## 引用计数（reference counting） 智能指针类型
* 通过记录所有者的数量，使一份数据被多个所有者同时持有
* 在没有任何所有者时自动清理数据

引用和智能指针区别
引用： 只借用数据
智能指针： 很多时候都拥有它所指向的数据

智能指针的实现
通常使用struct实现，并且实现了Deref和Drop这两个trait
* Deref trait: 允许智能指针struct的实例像引用一样使用
* Drop trait: 允许自定义当智能指针示例走出作用域时的代码
*/

/*
Box<T> 是最简单的智能指针：
* 允许在heap上存储数据（而不是stack）
* stack上是指向heap数据的指针
* 没有性能开销
* 没有其他额外功能

应用场景：
* 在编译时，某类型的大小无法确定。但使用该类型时，上下文却需要知道它的确切大小。
* 当你有大量数据，想移交所有权，但需要确保在操作时数据不会被复制。
* 使用某个值，只关心它是否实现特定的trait，而不关心它的具体类型

Box<T> 是一个指针，Rust知道它需要多少空间，
因为 指针的大小不会基于它所指向的数据的大小而变化
只提供了“间接”存储和heap内存分配的功能
实现了Deref trait 和Drop trait
*/

use crate::smart_points::List::{Cons, Nil};

pub fn main() {
    let b = Box::new(5);
    println!("b is {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
