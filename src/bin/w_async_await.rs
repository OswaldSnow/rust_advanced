/*
   使用.await关键字可以暂停当前异步函数的执行，
   并将控制权交回给运行时。这使得其他任务可以在等待期间继续执行。
   虽然看起来这与同步代码相似，但实际上它允许多个异步任务并发执行，而不是阻塞整个线程。

   一个异步函数a，一般情况下同时会有多个调用
   例如
   a(); // 暂且称为 1调用
   a(); // 暂且称为 2调用
   a(); // 暂且称为 3调用
   而在异步函数a内部代码中，后缀存在 .await 的代码，会等待相关代码执行完成
   例如 1调用中执行了2秒 但是这2秒内，2调用任务可以继续执行
*/
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    println!("{:?}", start_time);
    /*
    虽然每个任务都需要5秒才能完成，但由于它们是并发执行的
    所以总的等待时间仍然是5秒，而不是15秒。
     */
    let task1 = a(); // 1调用
    let task2 = a(); // 2调用
    let task3 = a(); // 3调用

    // 等待所有任务完成
    futures::join!(task1, task2, task3);

    let action_interval = start_time.elapsed();
    // 5.0s
    println!("action_interval is {:.1?}", action_interval);
}

async fn a() {
    println!("开始计算...");
    sleep(Duration::from_secs(5)).await; // 模拟耗时操作
    println!("计算完成!");
}
