use std::future::Future;
use std::pin::{Pin, pin};
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};

pub struct TimerFuture {
    pub ticks_remaining: usize,
}

impl Future for TimerFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.ticks_remaining == 0 {
            return Poll::Ready("Task completed!");
        }
        self.ticks_remaining -= 1;
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

// Dummy waker helper function for minimal executor loop
fn noop_waker() -> Waker {
    fn noop(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
    fn dummy_raw_waker() -> RawWaker {
        let vtable = &RawWakerVTable::new(clone, noop, noop, noop);
        RawWaker::new(std::ptr::null(), vtable)
    }
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

pub fn block_on<F: Future>(future: F) -> F::Output {
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    let mut pinned_future = pin!(future);

    loop {
        match pinned_future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => break value,
            Poll::Pending => {}
        }
    }
}

fn main() {
    let timer = TimerFuture { ticks_remaining: 3 };

    // Execute our async state machine via our mini-executor
    let result = block_on(timer);

    assert_eq!(result, "Task completed!");
    println!("Success! Custom async Future completed polling cycle.");
}