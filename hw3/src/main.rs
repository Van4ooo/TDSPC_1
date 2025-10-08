mod collatz;
mod parallel;
mod random_nums;

fn main() {
    let data = random_nums::gen_random_nums(1_000_000);
    let thread_pool = parallel::ThreadPool::new(16, data).unwrap();

    let rez = thread_pool.execute(|data| parallel::calc_steps(data));
    dbg!(rez);
}
