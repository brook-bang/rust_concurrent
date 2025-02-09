use std::sync::atomic::fence;
use std::time::Duration;
use std::{sync::atomic::AtomicBool, thread};
use std::sync::atomic::Ordering::Release;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::Ordering::Acquire;

static mut DATA: [u64; 10] = [0;10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE;10];

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i as usize] = data };
            READY[i as usize].store(true,Release);
        });
    }
    thread::sleep(Duration::from_millis(200));
    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));
    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe {
                    DATA[i]
                });
            }
        }
    }
}

fn some_calculation(x: u64) -> u64 {
    x * 10
}