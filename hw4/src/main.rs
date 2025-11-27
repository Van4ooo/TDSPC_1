use std::time;

mod collatz;
mod parallel;
mod random_nums;
mod shared_vec;
mod visual;

fn main() {
    let (total_number, num_threads) = (10_000_000, 8);

    let data = random_nums::gen_random_nums(total_number);
    let thread_pool = parallel::ThreadPool::new(num_threads, data);

    let start = time::Instant::now();
    let rez = thread_pool.execute(parallel::calc_steps);

    visual::display_results(rez, start.elapsed());
}
