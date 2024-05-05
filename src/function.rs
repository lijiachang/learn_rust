fn main() {
    another_func(88)
}

fn another_func(x: i32) {
    println!("the value is {x}")
}

fn expression() {
    let y = {
        let x = 1;
        x + 3 //不能加分号，这是一个语句
    };

    println!("y is {y}")
}

// 函数的返回值
// 在 -> 符号后面声明函数返回值的类型
// 返回值就是函数体里面最有一个表达式的值
// 若想提前返回就使用return关键字，并且指定一个值
fn plus_five(x:i32) -> i32{
    x + 5 //这个函数的最后一个表达式是5，所以返回5
    // 不能加分号，如果加了分号这个函数就会返回() 也就是空的tuple
}
fn test() {
    let x = plus_five(2);
    println!("x is {x}");
}