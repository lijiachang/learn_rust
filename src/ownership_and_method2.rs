/*
在语义上，将值传递给函数和把值赋值给变量是类似的；
将值传递给函数将发生移动或复制
*/

pub fn main() {
    let s = String::from("hello");
    let (s2, len) = calculate_length(s);
    println!("the length of '{s2}' is {len}");

}

fn calculate_length(string_: String) -> (String, usize) {
    let len = string_.len();
    return (string_, len)
}

