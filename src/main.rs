use std::thread;
use std::time::Duration;

fn task1(n: i32) {
    for i in 0..n {
        println!("[FIRST THREAD] {i}^3 = {}", i.pow(3));
        thread::sleep(Duration::from_secs(1));
    }
    println!("[FIRST THREAD] finished");
}

fn task2(n: i32) {
    for i in 0..n {
        println!("[SECOND THREAD] {i}^2 = {}", i.pow(2));
        thread::sleep(Duration::from_secs(1));
    }
    println!("[SECOND THREAD] finished");
}

fn main() {
    let t1 = thread::spawn(|| task1(5));
    let t2 = thread::spawn(|| task2(5));

    t1.join().expect("first thread failed");
    t2.join().expect("second thread failed");

    println!("\n[MAIN THREAD] finished");
}
