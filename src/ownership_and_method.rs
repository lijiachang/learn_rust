/*
在语义上，将值传递给函数和把值赋值给变量是类似的；
将值传递给函数将发生移动或复制
*/

fn main() {
    let s = String::from("hello");
    take_ownership(s); // 函数使用s后，后面的代码中s不可用了

    let x = 5;
    makes_copy(x); // 由于x是i32类型，有copy trait 函数使用x后，后面可以继续使用
    println!("x: {x}")
}

fn take_ownership(string_: String) {
    println!("{string_}")
}

fn makes_copy(number: i32) {
    println!("{number}")
}