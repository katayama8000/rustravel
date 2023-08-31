use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32);
impl Future for CountDown {
    type Output = String;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready("Zero!!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
fn main() {
    let count_down_future1: CountDown = CountDown(10);
    let count_down_future2: CountDown = CountDown(20);
    let cd_set: futures::future::JoinAll<CountDown> =
        join_all(vec![count_down_future1, count_down_future2]);
    let res: Vec<String> = executor::block_on(cd_set);
    println!("{:?}", res);
    // for (i, s) in res.iter().enumerate() {
    //     println!("{} {}", i, s);
    // }
}
