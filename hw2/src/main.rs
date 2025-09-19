mod monte_carlo;
mod point;
use std::time::{Duration, Instant};

fn print_result(name: &str, pi: f64, elapsed: Duration) {
    let true_pi = std::f64::consts::PI;
    let error = ((pi - true_pi).abs() / true_pi) * 100.0;

    println!(
        "{:<12} | pi ≈ {:<10.6} | error = {:<8.6} % | time = {:?}",
        name, pi, error, elapsed
    );
}

fn main() {
    let mut start = Instant::now();
    let mut pi = monte_carlo::linear(1_000_000);
    print_result("linear", pi, start.elapsed());

    for i in [2, 4, 8, 16, 32, 64, 128] {
        start = Instant::now();
        pi = monte_carlo::parallel(1_000_000, i);
        print_result(&format!("parallel-{i}"), pi, start.elapsed());
    }
}
