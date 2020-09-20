use rayon::prelude::*;
use anyhow::{ Result, ensure};

///
/// The function is_prime takes an integer as its argument
/// and returns true if it is prime and false if it is not.
///
/// It uses a brute force algorithm as the point of the
/// program is to be correct and parallelized.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = primes_lib::is_prime(arg);
///
/// assert_eq!(true, answer);
/// ```
pub fn is_prime(num: u64) -> bool {
    // Even though with this algorithm i don't think this would return
    // in a reasonable amount of time, it's worth it to prevent an overflow
    // just in case.
    let possible_factor = |x: u64| {
        let (val, overflowed) = x.overflowing_mul(x);
        val <= num && !overflowed
    };
    if num < 2 {
        return false;
    }
    // now we iterate over all possible factors that could be divisors.
    !(2..num)
        .take_while(|&x| possible_factor(x))
        .any(|x| num % x == 0)
}

///
/// The function prime returns a Result that contains
/// a vec of all u64 that are prime between start and stop.
///
/// It throws an error if
///
/// # Example
///
/// ```
/// let primes = primes_lib::primes(10, 20);
///
/// assert_eq!(vec![11, 13, 17, 19], primes.unwrap());
/// ```
pub fn primes(start: u64, stop: u64) -> Result<Vec<u64>> {
    // We check that stop is greater than start because
    // even though it would return a
    ensure!(stop >= start, "Start cannot be greater than stop.");
    // there are no primes less than 2 so need to
    // start there.

    let start = start.max(2);

    // here is where parallelism happens.
    Ok((start..=stop).into_par_iter()
        .filter_map(|n| {
            if is_prime(n) { Some(n) }
            else { None }
        }).collect())
}


#[cfg(test)]
mod tests {
    #[test]
    fn not_prime() {
        // A couple of tricky tests that are not prime
        assert_eq!(false, super::is_prime(1000));
        assert_eq!(false, super::is_prime(1));
        assert_eq!(false, super::is_prime(0));
    }
    #[test]
    fn is_prime() {
        // A couple of tricky tests that are primes
        assert_eq!(true, super::is_prime(2));
        assert_eq!(true, super::is_prime(3));
        assert_eq!(true, super::is_prime(31));
    }

    #[test]
    fn check_primes_func() {
        // not empty
        assert_eq!(vec![2, 3, 5, 7], super::primes(0, 7).unwrap());

        // empty because start == end and start is not prime
        assert_eq!(Vec::<u64>::default(), super::primes(10, 10).unwrap());

        // empty because no primes
        assert_eq!(Vec::<u64>::default(), super::primes(32, 36).unwrap());
    }


    #[test]
    #[should_panic]
    fn stop_less_than_start() {
        // not empty
        super::primes(10, 7).unwrap();
    }
}
