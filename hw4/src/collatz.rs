pub struct Collatz;

impl Collatz {
    #[inline]
    fn next(n: u128) -> Option<u128> {
        if n & 1 == 0 {
            Some(n / 2)
        } else {
            n.checked_mul(3)?.checked_add(1)
        }
    }

    pub fn steps(n: u64) -> Option<u128> {
        if n == 0 {
            return None;
        }

        let (mut steps, mut n) = (0, n as u128);
        while n != 1 {
            n = Self::next(n)?;
            steps += 1;
        }

        Some(steps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_numbers() {
        for (input, expected) in [
            (1, 0),
            (2, 1),
            (3, 7),
            (6, 8),
            (27, 111),
            (97, 118),
            (77_031, 350),
            (837_799, 524),
            (8_400_511, 685),
        ] {
            assert_eq!(
                Collatz::steps(input).unwrap(),
                expected,
                "Failed on input {}",
                input
            );
        }
    }

    #[test]
    fn invalid_numbers() {
        assert_eq!(Collatz::steps(0), None);
    }
}
