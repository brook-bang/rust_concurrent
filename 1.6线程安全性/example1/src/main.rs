use std::{cell::Cell, marker::PhantomData};

fn main() {
    println!("Hello, world!");
}
//X is not Sync or send because it contains a Cell
struct X {
    handle: i32,
    _not_sync: PhantomData<Cell<()>>
}