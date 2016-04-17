use std::vec::Vec;
use std::iter::repeat;
use std::cmp::max;
use std::cmp::min;

pub fn least_divisors_up_to(n : usize) -> (Vec<usize>, Vec<usize>) {
    let mut least_factor = Vec::with_capacity(n+1);
    least_factor.extend(repeat(0).take(n+1));
    let mut primes = Vec::new();

    for p in 2..n {
        if least_factor[p] == 0 {
            least_factor[p] = p;
            primes.push(p);
        }
        for prime in &primes {
            if *prime * p > n {
                break;
            }
            least_factor[p * *prime] = *prime;
            if least_factor[p] == *prime {
                break;
            }
        }

    }
    (least_factor, primes)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn small_primes_work() {
        let (array, _)  = least_divisors_up_to(10);

        assert_eq!(array[3], 3);
        assert_eq!(array[5], 5);
        assert_eq!(array[7], 7);
    }

    #[test]
    fn small_compounds_work() {
        let (array, _) = least_divisors_up_to(10);

        assert_eq!(array[4], 2);
        assert_eq!(array[6], 2);
        assert_eq!(array[9], 3);
    }

    #[test]
    fn returns_right_amount_of_small_primes() {
        let (_, primes) = least_divisors_up_to(10);
        assert_eq!(primes.len(), 4);
    }

    #[test]
    fn returns_right_small_primes() {
        let (_, primes) = least_divisors_up_to(10);
        assert_eq!(primes[0], 2);
        assert_eq!(primes[2], 5);
    }

    // 715 5417
    #[test]
    fn larger_primes() {
        let (_, primes) = least_divisors_up_to(10000);
        assert_eq!(primes[714], 5417); // 715th prime
        assert_eq!(primes[985], 7789); // 956th prime
    }

    #[test]
    fn right_amount_of_large_primes() {
        let (_, primes) = least_divisors_up_to(87522); // 87523 will be prime
        assert_eq!(primes.len(), 8495);
    }


}
