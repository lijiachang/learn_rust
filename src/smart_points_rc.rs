/*

Rc<T> ： 引用计数的智能指针
有时候，一个值会有多个所有者

为了支持多重所有权，所以有了RC<T> 也就是reference counting（引用计数）
当0个引用时，该值可以被清理掉

应用场景：
需要在heap上分配数据，这些数据被程序的多个部分读取（只读），但是在编译时无法确定哪个部分最后使用完这些数据

RC<T>智能用于单线程场景
RC<T>通过不可变引用，可以在程序不同的部分之间共享只读数据

RC::clone(&a) 增加引用计数  （和普通的clone方法区别：这个不会执行数据的深度拷贝操作，类型的clone方法，很多都会执行深拷贝）
Rc::strong_count(&a) 获得引用计数（强引用）


*/

use std::rc::Rc;
use crate::smart_points_rc::List::{Cons, Nil};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

pub fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c: {}", Rc::strong_count(&a));
    }
    // c离开组用于后，&a的引用计数会减少1

    println!("count after c goes out of scope:{}", Rc::strong_count(&a))
}