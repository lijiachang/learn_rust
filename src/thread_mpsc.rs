/*
使用消息传递来跨线程传递数据
Channel (标准库实现)
包含：发送端、接收端
调用发送端的方法发送数据，接收端会检查和接收到达的数据

收到的结果都是Result，如果有err那么说明Channel被关闭了
当发送端的所有channel都被销毁的时候，那么这个Channel就没了，接受者也就知道了

mpsc: multiple producer, single consumer
可以有多个生产者，但是只有一个消费者
*/

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let (tx, rx) = mpsc::channel();

    // 先克隆一个生产者
    let tx2 = mpsc::Sender::clone(&tx);
    // let tx2 = tx.clone();  这种写法应该也可以？

    //第一个生产者，在一个线程中
    thread::spawn(move || {
        let vals = vec![
            "1: hi".to_string(),
            "1: from".to_string(),
            "1: the".to_string(),
            "1: thread".to_string(),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    //第二个生产者，在另一个线程中
    thread::spawn(move || {
        let vals = vec![
            "2: hi".to_string(),
            "2: from".to_string(),
            "2: the".to_string(),
            "2: thread".to_string(),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

        // rx.recv().unwrap();
    // 把rx当做迭代器来使用，就不用调用rx.recv了
    for receiver in rx{
        println!("receiver get: {receiver}")
    }
}
