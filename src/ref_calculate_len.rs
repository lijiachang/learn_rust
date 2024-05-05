// 把引用作为函数参数的这个行为叫做借用
// 不可用修改借用的东西

pub fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //引用s1 但不拥有s1
    // 所以s1 走出作用域后，s1并不会被清理掉

    println!("the length of '{s1}' is {len}")
}

fn calculate_length(s: &String) -> usize {
    // &String 表示字符串的引用 在离开作用域时 不会销毁指向的数据 因为没有该所有权
    return s.len()
}
