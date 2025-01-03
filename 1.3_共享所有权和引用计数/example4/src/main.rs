use std::{sync::Arc, thread};

fn main() {
    let a = Arc::new([1,2,3]);
    let b = a.clone();
    thread::spawn(move|| dbg!(a)).join().unwrap();
    thread::spawn(move|| dbg!(b)).join().unwrap();
}
