use std::cell::Cell;

fn main() {
    let v = Cell::new(vec![1, 2, 3]);
    f(&v);
    println!("{:?}", v.take());
}

fn f(v: &Cell<Vec<i32>>) {
    let mut v2 = v.take();
    v2.push(1);
    v.set(v2);
}
