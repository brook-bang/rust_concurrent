use std::{rc::Rc, thread};

fn main() {
    let a = Rc::new(123);
    // 非send对象不能在线程间传递
    // thread::spawn(move || {
    //     dbg!(a);
    // });
}