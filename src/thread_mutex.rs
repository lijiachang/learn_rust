/*
共享状态的并发

之前介绍过用mpsc来实现线程的通讯
现在使用共享内存(共享状态)来实现线程并发
多个线程同时访问同个内存

为了实现安全，
使用 Mutex 来每次只允许一个线程来访问数据
Mutex是 mutual exclusion （互斥锁）的简写

在使用之前，必须先尝试获取锁（lock）
使用完mutex所保护的数据，必须对数据进行解锁，以便其他线程可以获取锁。


使用方法：
* 通过Mutex::new(数据)来创建Mutex<T>
    Mutex<T>是一个智能指针
* 访问数据钱，通过lock方法来获取锁
    会阻塞当前线程
    lock可能会失败
    返回的是MutexGuard（智能指针，实现了Deref和Drop）


多线程的多重所有权，需要使用ARC
使用Arc<T>来进行原子引用计数
* Arc<T>和Rc<T>类似，他可以用于并发情景。 A表示atomic原子性的


对比 RefCell<T> / Rc<T> 和 Mutex<T> / Arc<T>
* Mutex<T>提供了内部可变性，和Cell家族一样
* 我们使用RefCell<T> 来改变Rc<T> 里面的内容
* 我们使用Mutex<T> 来改变Arc<T> 里面的内容
* 注意使用 Mutex<T>有死锁风险
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn test_mutex() {
    let m = Mutex::new(5);

    {
        // num 是可变引用
        let mut num = m.lock().unwrap();
        *num =6;
    } // 离开作用域的时候，会自动解锁

    println!("m = {:?}", m)
}

fn test_thread_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10{
        let counter_ = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut c = counter_.lock().unwrap();
            *c += 1
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("counter={:?}", *counter.lock().unwrap())
}

pub fn main() {
    test_mutex();
    test_thread_mutex();
}