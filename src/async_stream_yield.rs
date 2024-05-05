/*

在 Rust 中，异步流（asynchronous streams）是一种可以逐项生成多个异步项的数据结构。
这些项可以在需要时懒加载，适用于处理连续的或未知数量的异步事件，如从网络接口接收数据或生成无限序列。
Rust 的异步流通过 Stream trait 来定义，而 stream! 宏和 yield 关键字则用于方便地创建和操作这些流。

stream! 宏
stream! 宏由 async-stream crate 提供，允许你以类似生成器（generator）的方式来定义异步流。这种方式通过编写一个异步块（使用 async 关键字），并在其中通过 yield 产生值，创建一个实现了 Stream trait 的对象。

yield 关键字
在 stream! 宏内部，yield 关键字用于向流中插入值。每次调用 yield 时，流的消费者就能接收到一个值。当流的生产者再次被唤醒并继续执行到下一个 yield 表达式时，它会生成下一个值。
*/


use async_stream::stream;
use futures::StreamExt;  // 引入 StreamExt trait 来使用 next 方法
use pin_utils::pin_mut;  // 引入 pin_mut 宏
use tokio::time::{self, Duration};

#[tokio::test]
async fn main() {
    let mut interval = time::interval(Duration::from_secs(1));
    let numbers = stream! {
        let mut count = 0;
        loop {
            interval.tick().await;
            yield count;
            count += 1;
            if count > 5 { break; }  // 只生成 6 个数字
        }
    };

    pin_mut!(numbers);  // 使用 pin_mut!(numbers); 来固定 numbers 流。这样做确保了流在内存中的位置不变，使得它满足 Unpin 特质的要求，从而允许调用 next() 方法。

    //  StreamExt::next() 方法需要被调用的流实例满足 Unpin 特质。解决方法是使用 Pin 来确保流对象不会在内存中被移动。
    while let Some(number) = numbers.next().await {
        println!("Received: {}", number);
    }
}
