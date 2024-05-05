/*

tokio::select! 是 Tokio 异步运行时提供的一个宏，用于同时等待多个异步任务或操作，并处理第一个完成的任务或操作。
这使得你可以同时处理多个不同来源的事件或数据流，并快速响应首先完成的那个。tokio::select! 特别适用于需要同时监听多个异步通道或超时等情况。

tokio::select! 宏允许你在几个不同的 Future 中选择首先完成的那个来继续执行。如果一个 Future 完成了，其他的 Future 将被取消。这对于实现超时或同时处理多个输入源特别有用。

以下是一个使用 tokio::select! 的简单示例。在这个示例中，我们将启动两个异步任务：一个简单的定时器和一个模拟数据处理任务。
我们将使用 tokio::select! 来等待这两个任务中任意一个首先完成：
*/

use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let sleep_future = time::sleep(Duration::from_secs(5));
    let task_future = async {
        // 模拟一些数据处理
        time::sleep(Duration::from_secs(3)).await;
        "Task finished"
    };

    tokio::select! {
        _ = sleep_future => {
            println!("The sleep future completed first.");
        }
        // result = task_future 表示 完成时的操作：当 task_future 完成时，它的结果（假设是 result）将被用于执行后面的代码块。
        // 代码块：=> 后面的 {...} 是一段代码块，将在前面的 Future 完成时执行。
        result = task_future => {
            println!("The task future completed first: {}", result);
        }
    }
    //使用 tokio::select! 时需要注意，一旦某个分支完成，其他所有未完成的分支会被取消。
    // 这意味着如果这些异步操作中包含了一些关键的清理或状态更新步骤，你需要确保它们能够在被取消时正确处理。
}

#[tokio::test]
async fn test_select_behavior() {
    let sleep_future = time::sleep(Duration::from_secs(5));
    let task_future = async {
        time::sleep(Duration::from_secs(3)).await;
        "Task finished"
    };

    let start = time::Instant::now();
    tokio::select! {
        _ = sleep_future => {
            assert!(false, "The sleep future should not complete first in this test.");
        }
        result = task_future => {
            println!("The task future completed first: {}", &result);
            assert_eq!(result, "Task finished");
        }
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed >= Duration::from_secs(3) && elapsed < Duration::from_secs(5),
        "Task should finish around 3 seconds."
    );
}
