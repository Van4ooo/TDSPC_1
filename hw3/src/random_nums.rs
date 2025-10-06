use std::collections::{LinkedList};
use rand::{thread_rng, Rng};

#[inline]
pub fn gen_random_nums(n: u64) -> LinkedList<u64>{
    let mut deque = LinkedList::new();
    let mut rng = thread_rng();

    for _ in 0..n {
        deque.push_back(rng.r#gen());
    }
    deque
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn check_size(){
        let deque = gen_random_nums(1_000);
        assert_eq!(deque.len(), 1_000);

        let deque = gen_random_nums(10_000_000);
        assert_eq!(deque.len(), 10_000_000);
    }
}