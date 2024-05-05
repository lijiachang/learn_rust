// 可变引用
// 可变引用有个限制：在特定作用域内，对某一块数据，只能有一个可变的引用
// 这样做的好处是在编译时防止数据竞争
/*以下三种行为下会发生数据竞争:
两个或多个指针同时访问同一个数据
至少有一个指针用于写入数据
没有使用任何机制来同步对数据的访问
*/
pub fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1); //引用s1 但不拥有s1
    // 所以s1 走出作用域后，s1并不会被清理掉

    println!("the length of '{s1}' is {len}")
}

fn calculate_length(s: &mut String) -> usize {
    // &String 表示字符串的引用 在离开作用域时 不会销毁指向的数据 因为没有该所有权
    s.push_str(", world");
    return s.len()
}

