/*
函数在返回值的过程中同样会发生所有权的转移
一个变量的所有权总是遵循同样的模式：

*/
fn main() {
    let s1 = gives_ownership();  // 把字符串移动给了s1

    let s2 = String::from("hello"); // 字面量创建一个字符串， s2进入到作用域
    let s3 = takes_and_gives_back(s2); // 然后移动到函数里面

    // 当main结束时，s3离开作用域，被drop销毁
}

fn gives_ownership() -> String {
    // 返回字符串的函数
    let string_ = String::from("Hello");
    string_
}

fn takes_and_gives_back(string_: String) -> String {
    // 取得一个string的所有权，并且返回这个string
    string_
}