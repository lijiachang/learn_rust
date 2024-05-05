struct Person {
    name : String,
    age : u8,
}

#[test]
fn main() {
    let p = Person{ name: "Hao Chen".to_string(), age : 44};

    //可以运行，因为 u8 有 Copy Trait
    let age = |p : &Person| p.age;

    // String 没有Copy Trait，所以，这里所有权就 Move 走了
    // let name = |p : &Person | &p.name;

    // 这里，我们使用 for<'a> 语法显式地量化了生命周期 'a。这意味着，对于任意生命周期 'a，只要输入的 Person 引用的生命周期是 'a，返回的 String 引用的生命周期也必须是 'a。
    // 通过这种方式，我们向 Rust 编译器保证，返回的引用的生命周期与输入的引用的生命周期相同，从而解决了潜在的生命周期不匹配问题。
    let name: for<'a> fn(&'a Person) -> &'a String = |p: &Person| &p.name;


    println! ("name={}, age={}" , name(&p), age(&p));
}