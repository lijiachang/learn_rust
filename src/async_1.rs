use std::cmp::PartialEq;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use tokio::time::Duration;

struct SleepFuture {
    duration: Duration,
    state: Arc<Mutex<State>>,
}

struct State {
    waker: Option<Waker>,
    inner_state: InnerState,
}

#[derive(PartialEq)]
enum InnerState {
    Init,
    Sleeping,
    Done,
}

impl SleepFuture {
    fn new(duration: Duration) -> SleepFuture {
        Self {
            duration,
            state: Arc::new(Mutex::new(State {
                waker: None,
                inner_state: InnerState::Init,
            })),
        }
    }
}

impl Future for SleepFuture {
    type Output = i16;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.state.lock().unwrap();
        println!("Polling...");

        // Ready状态
        if guard.inner_state == InnerState::Done {
            return Poll::Ready(666);
        }

        // 初始化
        if guard.inner_state == InnerState::Init {
            let waker = cx.waker().clone();
            guard.waker = Some(waker);

            guard.inner_state = InnerState::Sleeping;
            let state_clone = Arc::clone(&self.state);

            let duration = self.duration.clone();
            thread::spawn(move || {
                println!("start sleeping");
                thread::sleep(duration);
                let mut t_guard = state_clone.lock().unwrap();
                t_guard.inner_state = InnerState::Done;
                if let Some(waker) = t_guard.waker.take(){
                    waker.wake();
                    println!("wake up");
                }
            });
        }

        // guard.waker = Some(cx.waker().clone());

        // Pending状态
        Poll::Pending
    }
}

#[tokio::main]
pub(crate) async fn main() {
    println!("start in main");
    let f = SleepFuture::new(Duration::from_secs(2)).await;
    println!("future result is {f}");
    println!("stop in main");
}

// async await 原理，手写 async await 转换成的 future 代码
// https://www.bilibili.com/video/BV1PV411L7HN/?p=4&spm_id_from=pageDriver
// 更进一步的原理请看这个视频

// 其实就是多个Future的状态机会嵌套在一起，一层包一层，最终组成一个大的Future来提供给事件循环来调度
// 和Python一样的链式调用，