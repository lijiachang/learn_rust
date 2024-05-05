pub fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(&string1, &string2);
    println!("the longest is: {result}")
}

// 'a 生命周期就是表示x和y他俩最短的生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

use std::fmt::Display;

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


pub fn test_longest_with_announcement(){
    let x = String::from("abc");
    let y = String::from("b");
    let ann = String::from("2024-04-06 20:42:20");

    let result = longest_with_announcement(&x, &y, ann);
    println!("result is {result}")
}

/*
在 Rust 中，String 和 str 类型是两种常用的字符串类型。String 是一个可增长的、堆分配的字符串类型，而 str 通常以切片形式存在，是一个不可变的固定长度的字符串，在 Rust 中被称为字符串切片（string slice）。要将 String 手动转换为 str，通常会使用以下方法：

借用： 通过对 String 使用 & 操作符，可以得到一个指向 String 内容的 str 切片。这是最简单和最常用的方法。
let s = String::from("hello");
let slice: &str = &s;

调用 as_str 方法： String 类型提供了一个 as_str 方法，它会返回 String 内容的字符串切片。
let s = String::from("hello");
let slice: &str = s.as_str();

这两种方法都不会转移所有权，而是创建了一个指向原始 String 数据的不可变引用。在 Rust 中，转换 String 为 str 实际上是创建了一个指向 String 内部数据的引用，而不是创建了一个新的字符串对象。这种设计避免了不必要的数据复制，符合 Rust 的内存安全和效率的设计原则。
*/