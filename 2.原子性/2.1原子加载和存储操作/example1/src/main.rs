use std::{sync::atomic::{AtomicBool, Ordering}, thread};

fn main(){

    static STOP: AtomicBool = AtomicBool::new(false);

    let background = thread::spawn(||{
        while !STOP.load(Ordering::Relaxed){
            //do_work();
        }
    });
    
    for line in std::io::stdin().lines(){
        match line.unwrap().as_str(){
            "stop" => break,
            "hello" => println!("Hello!"),
            cmd => println!("Unknown command: {}", cmd),
        }
    };

    STOP.store(true, Ordering::Relaxed);

    background.join().unwrap();

}