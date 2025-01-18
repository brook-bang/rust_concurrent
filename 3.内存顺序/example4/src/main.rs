use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

fn main() {
    a1();
    a2();
    b();

}

static X: AtomicI32 = AtomicI32::new(0);

fn a1() {
    X.fetch_add(5, Relaxed);
}

fn a2() {
    X.fetch_add(10, Relaxed);
}

fn b() {
    let a = X.load(Relaxed);
    let b = X.load(Relaxed);
    let c = X.load(Relaxed);
    let d = X.load(Relaxed);
    println!("{} {} {} {}", a, b, c, d);
}
