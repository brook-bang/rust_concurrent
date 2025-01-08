use std::{sync::atomic::AtomicUsize, time::Duration,thread}; // 导入标准库中的 AtomicUsiz};

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, std::sync::atomic::Ordering::Relaxed);
                main_thread.unpark();
            }
        });
        loop {
            let n = num_done.load(std::sync::atomic::Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Working .. {n}/100 done");
            thread::park_timeout(Duration::from_secs(1));
        }
    });
    println!("Done!");
}
