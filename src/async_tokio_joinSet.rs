use tokio::task::{JoinSet, JoinError};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn main() {
    let mut join_set = JoinSet::new();

    // 启动第一个异步任务
    join_set.spawn(async {
        sleep(Duration::from_secs(2)).await;
        "Task 1 completed"
    });

    // 启动第二个异步任务
    join_set.spawn(async {
        sleep(Duration::from_secs(1)).await;
        "Task 2 completed"
    });

    // 等待所有任务完成
    //     JoinSet 不保证任务完成的顺序。在上面的例子中，即使第二个任务先启动且其延时较短，但 join_next 会返回第一个完成的任务的结果。
    //     当 JoinSet 中没有更多任务时，join_next 会返回 None。
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(output) => println!("Ok: {}", output),
            Err(e) => eprintln!("Task failed: {:?}", e),
        }
    }
}
