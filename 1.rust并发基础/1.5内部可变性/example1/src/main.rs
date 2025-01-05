use std::cell::Cell;

fn f(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);
    let after = a.get();
    if before != after {
        println!("a changed from {} to {}", before, after);
    }
}

fn main() {
    println!("Hello, world!");
    let a = Cell::new(1);
    let b = Cell::new(2);
    println!("a: {}", a.get());
    println!("b: {}", b.get());
    f(&a, &b);
    println!("a: {}", a.get());
    println!("b: {}", b.get());
}
