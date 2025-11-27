use crate::parallel::ErrorStatus;
use std::time::Duration;

pub fn display_results(results: Vec<Result<(u128, u128), ErrorStatus>>, duration: Duration) {
    let (total_steps, total_count, empty_threads) = summarize_results(&results);
    let avg = calculate_average(total_steps, total_count);

    print_summary(total_count, total_steps, avg, empty_threads, duration);
    print_distribution(&results);
}

fn summarize_results(results: &[Result<(u128, u128), ErrorStatus>]) -> (u128, u128, usize) {
    let mut total_steps = 0u128;
    let mut total_count = 0u128;
    let mut empty_threads = 0usize;

    for r in results {
        match r {
            Ok((steps, count)) => {
                total_steps += steps;
                total_count += count;
            }
            Err(ErrorStatus::EmptyDeque) => empty_threads += 1,
            Err(e) => eprintln!("⚠️ Thread error: {:?}", e),
        }
    }

    (total_steps, total_count, empty_threads)
}

fn calculate_average(total_steps: u128, total_count: u128) -> f64 {
    if total_count == 0 {
        0.0
    } else {
        total_steps as f64 / total_count as f64
    }
}

fn print_summary(
    total_count: u128,
    total_steps: u128,
    avg: f64,
    empty_threads: usize,
    duration: Duration,
) {
    println!("\n📊 Collatz Calculation Results");
    println!("----------------------------------------------------------");
    println!("🔹 Processed numbers     : {:>15}", total_count);
    println!("🔹 Total steps           : {:>15}", total_steps);
    println!("🔹 Average steps/number  : {:>15.6}", avg);
    println!("🔹 Empty threads         : {:>15}", empty_threads);
    println!("----------------------------------------------------------");
    println!("⏱️  Execution time: {:.2?}", duration);
    println!("==========================================================");
}

fn print_distribution(results: &[Result<(u128, u128), ErrorStatus>]) {
    println!("\n📈 Per-thread load distribution:");

    let max_count = results
        .iter()
        .filter_map(|r| r.as_ref().ok().map(|(_, c)| *c))
        .max()
        .unwrap_or(1);

    for (i, res) in results.iter().enumerate() {
        match res {
            Ok((steps, count)) => print_thread_bar(i, *steps, *count, max_count),
            Err(_) => println!("Thread {:>2}: empty", i + 1),
        }
    }
}

fn print_thread_bar(index: usize, steps: u128, count: u128, max_count: u128) {
    let bar_len = ((count as f64 / max_count as f64) * 40.0).round() as usize;
    let avg_steps = steps / count.max(1);

    println!(
        "Thread {:>2}: {:>10} numbers | {:>12} steps | {:<40} avg {:>5}",
        index + 1,
        count,
        steps,
        "█".repeat(bar_len),
        avg_steps
    );
}
