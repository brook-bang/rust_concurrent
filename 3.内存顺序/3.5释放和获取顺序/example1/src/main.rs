use std::{sync::atomic::{AtomicBool, AtomicU64}, thread, time::Duration};
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::Ordering::Acquire;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);
fn main() {
    thread::spawn(||{
        DATA.store(123, Relaxed);
        READY.store(true, Release);
    });
    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}",DATA.load(Relaxed));
}
