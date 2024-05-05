use std::thread;

#[test]
fn main() {
    // 定义了一个 x + y 操作的 lambda f(x, y) = x + y;
    let plus = |x: i32, y: i32| x + y;
    // 定义另一个lambda g(x) = f(x, 5)
    let plus_five = |x| plus(x, 5);
    //输出
    println!("plus_five(10)={}", plus_five(10));
}

struct Person {
    name: String,
    age: u8,
}

#[test]
fn test_ownership() {
    let p = Person {
        name: "Hao Chen".to_string(),
        age: 44,
    };

    //可以运行，因为 u8 有 Copy Trait
    let age = |p: &Person| p.age;

    // 这里，我们使用 for<'a> 语法显式地量化了生命周期 'a。
    // 这意味着，对于任意生命周期 'a，只要输入的 Person 引用的生命周期是 'a，返回的 String 引用的生命周期也必须是 'a。
    // 通过这种方式，我们向 Rust 编译器保证，返回的引用的生命周期与输入的引用的生命周期相同，从而解决了潜在的生命周期不匹配问题。
    let name: for<'a> fn(&'a Person) -> &'a String = |p: &Person| &p.name;

    println!("name={}, age={}", name(&p), age(&p));
}

#[test]
fn test_ownership2() {
    let s = String::from("string");

    let take_str = || s;
    // take_str  把 s 的所有权给拿走了（因为需要作成返回值）。所以，后面的输出语句就用不到了。
    // 对于内建的类型，都实现了 Copy 的 trait，那么闭包执行的是 “借用”
    // 对于没有实现 Copy 的trait，在闭包中可以调用其方法，是“借用”，但是不能当成返回值，当成返回值了就是“移动”。
    // 虽然有了这些“通常情况下是借用的潜规则”，但是还是不能满足一些情况，所以，还要让程序员可以定义 move 的“明规则”。

    // println!("{}", s); //ERROR
    println!("{}", take_str()); // OK
}

#[test]
fn test_ownership3() {
    //-----------借用的情况-----------
    let mut num = 5;
    {
        let mut add_num = |x: i32| num += x;
        add_num(5);
    }
    println!("num={}", num); //输出 10

    //-----------Move的情况-----------
    let mut num = 5;
    {
        // 把 num（5）所有权给 move 到了 add_num 中，
        // 使用其成为闭包中的局部变量。
        let mut add_num = move |x: i32| num += x;
        //int这样的类型，因为实现了Copy Trait，所以所有权被移走后，意味着，在内嵌块中的num 和外层的 num 是两个完全不相干的变量。
        add_num(5);
        println!("num(move)={}", num); //输出10
    }
    //因为i32实现了 Copy，所以，这里还可以访问
    println!("num(move)={}", num); //输出5
}

#[test]
fn test_thread() {
    let name = "CoolShell".to_string();

    let t = thread::spawn(move || {
        println!("Hello, {}", name);
    });

    println!("wait {:?}", t.join()); //wait Ok(())
}

#[test]
fn test_thread2() {
    let name = "CoolShell".to_string();
    let mut threads = vec![];


    let name1 = name.clone();
    let t1 = thread::spawn(move || {
        println!("Hello, {}", name1);
    });

    let name2 = name.clone();
    let t2 = thread::spawn(move || {
        println!("Hello, {}", name2);
    });

    threads.push(t1);
    threads.push(t2);

    for t in threads {
        println!("wait {:?}", t.join());
    }
}