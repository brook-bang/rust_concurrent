use std::thread;

fn main() {
    static X: [i32;3] = [1,2,3];
    thread::spawn(|| dbg!(&X)).join().unwrap();
    thread::spawn(|| dbg!(&X)).join().unwrap();

}
