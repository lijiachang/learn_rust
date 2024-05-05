/*
内部可变性（interior mutability）
* 内部可变性是Rust设计模式之一
* 他允许你在只持有不可变引用的前提下对数据进行修改
    数据结构中使用了unsafe代码来绕过Rust正常的可变性和借用规则


https://www.bilibili.com/video/BV1hp4y1k7SV/
还没看懂，等待二刷，先跳过
大概就是 把内部的不可变对象变为可变
在运行时，而不是编译时

还有其他的可实现内部可变性的类型
* Cell<T>: 通过复制来访问数据
* Mutex<T>: 用户实现跨线程情形下的内部可变性模式


https://www.bilibili.com/video/BV1hp4y1k7SV
RefCell可能导致内存泄露, 相互引用
*/