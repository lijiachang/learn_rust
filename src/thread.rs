use std::thread;
use std::time::Duration;

pub fn main() {
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("number {} in spawn thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("number {} in main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join方法会阻止当前运行线程的执行，等待这个子线程结束
    handle.join().unwrap();
}

pub fn move_() {
    let v = vec![1, 2, 3];

    // 使用move 把v的所有权移动到子线程中
    let handle = thread::spawn(move || {
        println!("spawn thread v is {:?}", v)
    });

    // println!("main thread v is {:?}", v)
    // 这里就获取不到了
}