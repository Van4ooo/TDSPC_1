use rand::{Rng, thread_rng};

#[inline]
pub fn gen_random_nums(n: u64) -> Vec<u64> {
    let mut vec = Vec::with_capacity(n as usize);
    let mut rng = thread_rng();

    for _ in 0..n {
        vec.push(rng.r#gen());
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_size() {
        let vec = gen_random_nums(1_000);
        assert_eq!(vec.len(), 1_000);

        let vec = gen_random_nums(100_000);
        assert_eq!(vec.len(), 100_000);
    }
}
