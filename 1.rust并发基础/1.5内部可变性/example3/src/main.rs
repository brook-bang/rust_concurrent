use std::cell::RefCell;

fn main() {
    let v = RefCell::new(vec![1, 2, 3]);
    f(&v);
}

fn f(v: &RefCell<Vec<i32>>) {
    v.borrow_mut().push(5);
    println!("{:?}", v.borrow());
}
