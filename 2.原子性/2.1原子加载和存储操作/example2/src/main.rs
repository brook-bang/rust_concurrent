use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::time::Duration;

fn main() {
    let num_done = AtomicUsize::new(0);
    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                // Do some work
                thread::sleep(Duration::from_secs(1));
                num_done.store(i + 1, Relaxed);
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {break;}
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_secs(1));
        }
    });
    println!("Done!");
}
