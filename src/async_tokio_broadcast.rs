/*

broadcast

broadcast 是 tokio 中的一个模块，提供了一个多生产者、多消费者的广播通道。这种通道允许多个发送者和接收者之间的消息传递。每个发送到通道的消息都会被发送给所有当前的接收者。



*/


//在这个示例中，一个发送任务向 broadcast 通道发送了一系列整数。
// 有两个接收任务，每个都从同一个通道接收数据。
// 由于是广播通道，发送到通道的每个消息都会被两个接收者收到。

use tokio::sync::broadcast;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn main() {
    let (tx, mut rx1) = broadcast::channel::<i32>(16);
    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        for i in 0..5 {
            if tx.send(i).is_err() {
                println!("Error sending message");
                break;
            }
            sleep(Duration::from_millis(100)).await;
        }
    });

    tokio::spawn(async move {
        while let Ok(i) = rx1.recv().await {
            println!("rx1 received: {}", i);
        }
    });

    tokio::spawn(async move {
        while let Ok(i) = rx2.recv().await {
            println!("rx2 received: {}", i);
        }
    });

    sleep(Duration::from_secs(1)).await; // 等待足够时间以确保所有消息都已处理
}
