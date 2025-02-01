use std::time::Duration;
use std::{sync::atomic::AtomicBool, thread};
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::Ordering::Acquire;

static mut DATA: u64 = 0;
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(||{
        unsafe {DATA = 123};
        READY.store(true, Release);
    });
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}",unsafe {
        DATA
    });

}