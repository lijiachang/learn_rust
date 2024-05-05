/*
Option 枚举
定义于标准库中
在Prelude（预导入模块）中
描述了某个值可能存在（某种类型）或不存在的情况
*/

//Rust 没有Null
//Rust中有类似Null 概念的枚举：Option<T>

pub fn main() {
    let some_number = Some(5); //Some表示有值
    let some_string = Some("A String");
    let absent_number: Option<i32> = None;  //可能为空，没有一个有效的值
}