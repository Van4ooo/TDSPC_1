use crate::collatz::Collatz;
use std::collections::LinkedList;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadPool<T> {
    shared_data: Arc<Mutex<T>>,
    num_threads: usize,
}

impl<T: Send + 'static> ThreadPool<T> {
    pub fn new(nt: usize, data: T) -> Option<Self> {
        if nt == 0 {
            return None;
        }

        Some(Self {
            shared_data: Arc::new(Mutex::new(data)),
            num_threads: nt,
        })
    }

    pub fn execute<F, R, E>(&self, func: F) -> Vec<Result<R, E>>
    where
        F: Fn(Arc<Mutex<T>>) -> Result<R, E> + Clone + Send + 'static,
        R: Send + 'static,
        E: Send + 'static,
    {
        let mut handles = Vec::with_capacity(self.num_threads);
        for _ in 0..self.num_threads {
            let data = Arc::clone(&self.shared_data);
            let cloned_func = func.clone();
            handles.push(thread::spawn(move || cloned_func(data)));
        }

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum ErrorStatus {
    Mutex,
    StepOverflow { step: u64 },
    EmptyDeque,
}

pub fn calc_steps(deque: Arc<Mutex<LinkedList<u64>>>) -> Result<(u128, u128), ErrorStatus> {
    let mut total_steps = 0u128;
    let mut count = 0u128;

    loop {
        let el = {
            let mut lock_deque = deque.lock().map_err(|_| ErrorStatus::Mutex)?;
            (*lock_deque).pop_front()
        };

        if let Some(el) = el {
            count += 1;
            total_steps += Collatz::steps(el).ok_or(ErrorStatus::StepOverflow{step: el})?;
        } else {
            return if count == 0 {
                Err(ErrorStatus::EmptyDeque)
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
    fn execute_check() {
        let data = 1;
        let thread_pool = ThreadPool::new(4, data).unwrap();

        let rez_rev = thread_pool.execute(|data| {
            let mut ptr = data.lock().unwrap();
            *ptr += 1;
            return Ok(*ptr);
        });

        let mut rez: Vec<_> = rez_rev.into_iter().map(|el: thread::Result<_>| el.unwrap()).collect();
        rez.sort();

        assert_eq!(rez, vec![2, 3, 4, 5]);
    }

    #[test]
    fn empty_deque() {
        let data = LinkedList::new();
        let tread_pool = ThreadPool::new(4, data).unwrap();

        let rez = tread_pool.execute(|data| calc_steps(data));
        for nth in rez{
            assert_eq!(nth.err().unwrap(), ErrorStatus::EmptyDeque);
        }
    }

    #[test]
    fn check_avg_steps(){
        let data = LinkedList::from([27, 97, 77_031, 3, 6, 6, 8_400_511]);
        let thread_pool = ThreadPool::new(4, data).unwrap();

        let rez = thread_pool.execute(|data| calc_steps(data));
        let rez: Vec<(u128, u128)> = rez.into_iter().map(|n| n.unwrap_or_default()).collect();

        assert_eq!(rez.iter().map(|(s, _)| *s as u32).sum::<u32>(), 1287);
        assert_eq!(rez.iter().map(|(_, c)| *c as u32).sum::<u32>() , 7);

        dbg!(rez);
    }

    #[test]
    fn check_par(){
        let mut data = LinkedList::new();
        for _ in 0..100_000{
            data.push_back(8_400_511);
        }
        let thread_pool = ThreadPool::new(4, data).unwrap();

        let rez = thread_pool.execute(|data| calc_steps(data));
        let rez: Vec<(u128, u128)> = rez.into_iter().map(|n| n.unwrap_or_default()).collect();

        assert_eq!(rez.iter().map(|(s, _)| *s as u32).sum::<u32>(), 68_500_000);
        assert_eq!(rez.iter().map(|(_, c)| *c as u32).sum::<u32>() , 100_000);

        dbg!(rez);
    }
}
