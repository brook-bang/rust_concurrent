use std::sync::atomic::AtomicI32;

fn main() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, std::sync::atomic::Ordering::Relaxed);
    let c = a.load(std::sync::atomic::Ordering::Relaxed);
    println!("b: {}, c: {}", b, c);
}
