/*
Deref Trait 就是解引用的意思
如果实现了deref使得我们可以自定义解引用运算符*的行为

函数和方法的隐式解引用转换（Deref Coercion）
* 隐式解引用转换是为函数和方法提供的一种便捷特性
* 当把某类型的引用传递给函数或方法时，但它的类型和定义的参数类型不匹配
    Deref Coercion就会自动发生
    编译器会对deref进行一系列调用，来把它转换为所需的参数类型。
    在编译时就会完成，没有额外性能开销


解引用与可变性
可以使用DerefMut trait重载可变引用*运算符


*/

use std::ops::Deref;

fn test() {
    let x = 5;
    let y = &x; // y是x的引用, y就相当于一个指针

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用
}

fn test_box() {
    let x = 5;
    let y = Box::new(x); // Box<T>可以像引用一样处理

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用
}

fn test_my_box() {
    struct MyBox<T>(T);  // Box<T> 被定义为拥有一个元素的tuple struct 元祖结构体

    impl <T> MyBox<T>{
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 实现解引用*符号的功能
    impl <T> Deref for MyBox<T>{
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用
    // *y 实际上Rust会隐式的展开为 *(y.deref())
}

fn test_deref_coercion() {
    struct MyBox<T>(T);  // Box<T> 被定义为拥有一个元素的tuple struct 元祖结构体

    impl <T> MyBox<T>{
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    // 实现解引用*符号的功能
    impl <T> Deref for MyBox<T>{
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("hello, {name}");
    }

    let m = MyBox::new(String::from("rust"));
    hello(&m);
    // &m 这里会进行两次解引用
    // MyBox 实现了deref，所以&MyBox<String> 会得到&String
    // 然后&String 中的String也实现了deref，会得到&str
}
pub fn main() {
    test();
    test_box();
    test_my_box();
    test_deref_coercion();
}
