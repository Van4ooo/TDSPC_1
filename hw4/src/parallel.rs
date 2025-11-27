use crate::{collatz::Collatz, shared_vec::Shared};
use std::sync::Arc;
use std::thread;

pub struct ThreadPool<T> {
    shared_vec: Arc<Shared<T>>,
    num_threads: usize,
}

impl<T: Sync + Send + Copy + 'static> ThreadPool<T> {
    pub fn new(num_threads: usize, vec: Vec<T>) -> Self {
        Self {
            shared_vec: Arc::new(Shared::new(vec)),
            num_threads,
        }
    }

    pub fn execute<F, R, E>(&self, func: F) -> Vec<Result<R, E>>
    where
        F: Fn(Arc<Shared<T>>) -> Result<R, E> + Clone + Send + 'static,
        R: Send + 'static,
        E: Send + 'static,
    {
        let mut handles = Vec::with_capacity(self.num_threads);
        for _ in 0..self.num_threads {
            let data = Arc::clone(&self.shared_vec);
            let cloned_func = func.clone();
            handles.push(thread::spawn(move || cloned_func(data)));
        }

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ErrorStatus {
    StepOverflow { step: u64 },
    Empty,
}

pub fn calc_steps(vec: Arc<Shared<u64>>) -> Result<(u128, u128), ErrorStatus> {
    let mut total_steps = 0u128;
    let mut count = 0u128;

    loop {
        let el = vec.get();
        if let Some(el) = el {
            count += 1;
            total_steps += Collatz::steps(el).ok_or(ErrorStatus::StepOverflow { step: el })?;
        } else {
            return if count == 0 {
                Err(ErrorStatus::Empty)
            } else {
                Ok((total_steps, count))
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_vec() {
        let vec = vec![];
        let tread_pool = ThreadPool::new(4, vec);

        let rez = tread_pool.execute(calc_steps);
        for nth in rez {
            assert_eq!(nth.err().unwrap(), ErrorStatus::Empty);
        }
    }

    #[test]
    fn check_avg_steps() {
        let data = vec![27, 97, 77_031, 3, 6, 6, 8_400_511];
        let thread_pool = ThreadPool::new(4, data);

        let rez = thread_pool.execute(calc_steps);
        let rez: Vec<(u128, u128)> = rez.into_iter().map(|n| n.unwrap_or_default()).collect();

        assert_eq!(rez.iter().map(|(s, _)| *s as u32).sum::<u32>(), 1287);
        assert_eq!(rez.iter().map(|(_, c)| *c as u32).sum::<u32>(), 7);

        dbg!(rez);
    }

    #[test]
    fn check_par() {
        let data = vec![8_400_511; 100_000];
        let thread_pool = ThreadPool::new(4, data);

        let rez = thread_pool.execute(calc_steps);
        let rez: Vec<(u128, u128)> = rez.into_iter().map(|n| n.unwrap_or_default()).collect();

        assert_eq!(rez.iter().map(|(s, _)| *s as u32).sum::<u32>(), 68_500_000);
        assert_eq!(rez.iter().map(|(_, c)| *c as u32).sum::<u32>(), 100_000);

        dbg!(rez);
    }
}
