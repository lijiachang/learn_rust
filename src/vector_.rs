/*
Vec<T> 叫做vector
由标准库提供
可以存储多个值
只能存储相同类型的数据
值在内存中连续存放*/

use std::io::empty;

pub fn main() {
    // let v: Vec<i32> = Vec::new();  //标准创建
    // let v = vec![1, 2, 3]; //使用宏创建，自动推断类型

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
}

fn read_vector() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];  //使用切片读取
    println!("the third element is {}", third);

    //使用get读取
    match v.get(2) {  //可以超出索引
        Some(e) => println!("get third element{}", e),
        None => println!("no third element"),
    }
}

fn for_loop_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }
}

// 遍历的时候，修改值
pub fn update_element_when_for_loop_vector() {
    let mut v = vec![1, 2, 3];
    for i in &mut v { // &符号是取引用
        *i += 10; // *i是解引用，取到真实地址
    }

    for i in &mut v {
        println!("i is {i}")
    }
}

//结合枚举
enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vector() {
    let row = vec![
        SpreadSheetCell::Int(21),
        SpreadSheetCell::Float(22.222),
        SpreadSheetCell::Text(String::from("name")),
    ];
}