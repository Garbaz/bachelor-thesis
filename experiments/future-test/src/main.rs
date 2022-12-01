use std::{
    future::Future,
    mem::transmute,
    pin::Pin,
    ptr::NonNull,
    task::{self, Context, Poll, RawWaker, RawWakerVTable, Waker},
};

use rand::prelude::*;

#[derive(Debug)]
struct Trace(Vec<String>);

struct TwoPoint<T>(T, T)
where
    T: Clone;

impl<T> Future for TwoPoint<T>
where
    T: Clone + std::fmt::Display,
{
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Self::Output> {
        // println!("TestDist polled!");

        // cx.waker().wake_by_ref();

        let cx: &mut EvilContext = unsafe { transmute(cx) };

        // println!("{:?}", cx);

        let result = if random() {
            self.0.clone()
        } else {
            self.1.clone()
        };

        cx.0.0.push(format!("TwoPoint({},{}) -> {}", self.0, self.1, result));

        Poll::Ready(result)
    }
}

// async fn dist() -> f64 {
//     0.
// }

async fn test() -> u64 {
    let mut y = 0;
    for _ in 0..10 {
        y += TwoPoint(17, 29).await;
    }
    y
}

/* unsafe fn v_clone(p: *const ()) -> RawWaker {
    println!("clone");
    RawWaker::new(p, &RAW_WAKER_VTABLE)
}

unsafe fn v_wake(p: *const ()) {
    println!("wake");
    let mut data = NonNull::new_unchecked(p as *mut WakerData);
    let d = data.as_mut();
    d.0 += 1;
}

unsafe fn v_wake_by_ref(p: *const ()) {
    println!("wake ref");
    let mut data = NonNull::new_unchecked(p as *mut WakerData);
    let d = data.as_mut();
    d.0 += 1000;
}

unsafe fn v_drop(p: *const ()) {
    println!("drop");
}

static RAW_WAKER_VTABLE: RawWakerVTable =
    RawWakerVTable::new(v_clone, v_wake, v_wake_by_ref, v_drop);

#[derive(Debug)]
struct WakerData(u64); */

#[derive(Debug)]
struct EvilContext(Trace);

fn main() {
    let mut a = test();
    let b = unsafe { Pin::new_unchecked(&mut a) };

    /* let mut data = WakerData(0);
    let nn = unsafe { NonNull::new_unchecked(&mut data as *mut WakerData) };

    let w = unsafe {
        Waker::from_raw(RawWaker::new(
            nn.as_ptr() as *const (),
            &RAW_WAKER_VTABLE,
        ))
    };
    let mut c = Context::from_waker(&w); */

    let mut ecx = EvilContext(Trace(Vec::new()));

    // let p = b.poll(&mut c);
    let p = b.poll(unsafe { transmute(&mut ecx) });

    // println!("{:?}", data);
    println!("{:?}", p);
    println!("{:#?}", ecx);
}
