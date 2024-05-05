/*
通过Send和Sync Trait来拓展并发
* Rust的并发特性较少，目前讲的的并发特性都来自标准库，而不是来自语言本身
* 无需局限于标准库的并发，可以自己实现并发

在Rust语言中有两个并发概念的trait：
* std::marker::Sync
* std::marker::Send

Send: 允许线程间转移所有权
* 实现Send Trait的类型可以在线程间转移所有权
* Rust中几乎所有类型都实现了Send
    但是Rc<T>没有实现Send，所以它只能用户单线程
* 任何完全由Send类型组成的类型也被标记为Send
* 除了原始指针之外，几乎所有的基础类型都是Send

Sync: 允许从多线程访问
* 实现Sync的类型可以安全的被多个线程引用
* 也就是：如果T是Sync，那么&T就是Send
    引用可以被安全的送往另一个线程
* 基础类型都是Sync
* 完全由Sync类型组成的类型也是Sync
    Rc<T> 不是Sync的
    RefCell<T> 和 Cell<T> 家族也不是Sync的
    Mutex<T> 是Sync的


手动来实现Send和Sync是不安全的

*/