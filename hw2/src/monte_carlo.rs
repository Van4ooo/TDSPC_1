use crate::point::Point;
use std::thread;

fn core(n: i64) -> i64 {
    let mut rng = rand::thread_rng();

    let mut point;
    let mut in_cycle = 0;

    for _ in 0..n {
        point = Point::new_gen(&mut rng);
        in_cycle += point.in_circle() as i64;
    }

    in_cycle
}

#[inline]
pub fn linear(n: i64) -> f64 {
    4f64 * (core(n) as f64 / n as f64)
}

pub fn parallel(n: i64, threads_count: u8) -> f64 {
    let portion = n / threads_count as i64;

    let mut handles = vec![];
    for _ in 0..threads_count {
        handles.push(thread::spawn(move || core(portion)));
    }

    let mut in_cycle = 0;
    for th in handles {
        in_cycle += th.join().expect("[ERROR] thread failed");
    }

    4f64 * (in_cycle as f64 / n as f64)
}
