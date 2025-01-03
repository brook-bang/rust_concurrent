fn f(a: &i32,b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;
    if before != after {
        x();
    }

}
fn main() {
    let mut x = 0;
    let y = 0;
    f(&y, &mut x);
    println!("{}", x);
}

fn x() {
    println!("之前不等于之后");
}