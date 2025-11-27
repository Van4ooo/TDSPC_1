use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Shared<T> {
    vec: Vec<T>,
    ptr: AtomicUsize,
}

impl<T: Copy> Shared<T> {
    #[inline]
    pub fn new(vec: Vec<T>) -> Self {
        Self {
            vec,
            ptr: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn get(&self) -> Option<T> {
        let now_el = self.ptr.fetch_add(1, Ordering::Relaxed);

        if now_el >= self.vec.len() {
            return None;
        }

        // SAFE: always valid index
        Some(unsafe { *self.vec.get_unchecked(now_el) })
    }
}

#[cfg(test)]
mod tests {
    use super::Shared;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_get() {
        let vec = Vec::from_iter(2u64..100_000);
        let sum_expected: u64 = vec.iter().sum();

        let sh_vec = Arc::new(Shared::new(vec));

        let mut handles = vec![];
        for _ in 0..16 {
            let sh_vec_clone = Arc::clone(&sh_vec);

            handles.push(thread::spawn(move || {
                let mut rez = Some(0);
                let mut count_rez = 0;

                while rez.is_some() {
                    rez = sh_vec_clone.get();
                    if let Some(r) = rez {
                        count_rez += r;
                    }
                }

                count_rez
            }));
        }

        let rez: u64 = handles.into_iter().map(|el| el.join().unwrap()).sum();

        assert_eq!(rez, sum_expected);
    }
}
