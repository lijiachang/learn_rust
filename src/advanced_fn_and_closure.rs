/*

高级函数和闭包(匿名函数)

函数指针
* 可以将函数传递给函数
* 函数在传递过程过程中会被强制转为fn类型
* fn类型就是“函数指针（function pointer）”

*/

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn test_do_twice() {
    let result = do_twice(add_one, 5);
    assert_eq!(result, 12);
}

pub fn main() {
    test_do_twice();
}

/*

函数指针与闭包的不同
* fn是一个类型，不是一个trait
* 函数指针实现了全部3种闭包trait（Fn、FnMut、FnOnce）
    可以把函数指针用作参数传递给一个接受闭包的函数
    所以，倾向于搭配闭包trait的泛型来编写函数：可以同时接受闭包和普通函数

在某些情景，只想接受fn而不接收闭包：
* 与外部不支持闭包的代码交互：C函数

*/

fn vec_to_string() {
    // 使用闭包（匿名函数）实现
    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_numbers.iter().map(|x| x.to_string()).collect();

    //使用函数（指针）实现
    let list_of_numbers = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_string.iter().map(ToString::to_string).collect();
}

fn u32_to_enum() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    // (0u32..20)是一个范围表达式（Range），表示从0到19的一系列数值，包含起始值0，但不包含结束值20。
    // 这个表达式生成了一个迭代器，该迭代器依次产生从0到19的每个u32整数值。
}

/*
返回闭包
闭包使用trait进行表达，无法在函数中直接返回一个闭包，可以将一个实现了该trait的具体类型作为返回值。

*/

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
