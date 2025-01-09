use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    println!("Hello, world!");
}

fn increment(a: &AtomicU32) {
    let mut current = a.load(Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Relaxed, Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}