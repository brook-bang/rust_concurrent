fn main() {
    println!("Hello, world!");
}

fn f(a:&mut i32, b: &mut i32) {
    *a += 1;
    *b += 1;
    *a += 1;
}
