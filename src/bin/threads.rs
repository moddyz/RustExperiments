use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("# {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join();

    for i in 1..5 {
        println!("# {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Move ownership of v into the closure executed by the thread.
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("a vector: {:?}", v);
    });
    handle.join();
}
