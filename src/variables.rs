const NUM:i32 = 100_000; //常量，必须指定类型

fn main() {
    let x = 5;  //不可变变量
    println!("the value is {x}");

    let mut y = 6;  //可变的变量
    let y = 8;
    println!("y is {y}");

    //元祖
    let tup:(i32, f64, u8) = (500, 2.3, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    //解构
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    //数组：每个元素的类型必须相同；长度是固定的
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months = ["Jan", "Feb", "Mar"];
    let first = months[0];
    let second = months[1];




}