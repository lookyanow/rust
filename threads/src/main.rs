use std::thread;
use std::time::Duration;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("this is number: {} from spawned thread1", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 1..50 {
            println!("this is number: {} from second thread2", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    handle1.join().unwrap();
    handle2.join().unwrap();
    for i in 1..5 {
        println!("this is number: {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

}
