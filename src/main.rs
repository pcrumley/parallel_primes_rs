use clap::{App, Arg};
use rayon::prelude::*;
use anyhow::{Context, Result, ensure};
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
/// let answer = parallel_primes_rs::add_one(arg);
///
/// assert_eq!(true, answer);
/// ```
fn is_prime(num: u64) -> bool {
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

fn primes(start: u64, stop: u64) -> Result<Vec<u64>> {
    // We check that stop is greater than start because
    // even though it would return a
    ensure!(stop >= start, "Start cannot be greater than stop.");
    // there are no primes less than 2 so need to
    // start there.
    let start = start.max(2);
    Ok((start..=stop).into_par_iter()
        .filter_map(|n| {
            if is_prime(n) { Some(n) }
            else { None }
        }).collect())
}

fn main() -> Result<()> {
    // Use the Clap.rs argument parser to capture user inputs. Does nice job of adding
    // a help flag & throws returns an error if required args are missing.
    let matches = App::new("Parallel Primes")
                          .version("1.0")
                          .author("Patrick C. <patrick.crumley@gmail.com>")
                          .about("Prints an array of prime numbers between start and stop (inclusive)")
                          .arg(Arg::with_name("THREADS")
                               .short("N")
                               .long("Nthreads")
                               .value_name("NTHREADS")
                               .help("Sets the number of threads")
                               .takes_value(true))
                          .arg(Arg::with_name("START")
                               .help("The number we start looking for primes")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("STOP")
                                .help("Sets the input file to use")
                                .required(true)
                                .index(2))
                          .get_matches();

    if let Some(n) = matches.value_of("THREADS") {
        let n = n.parse::<usize>()
                    .with_context(|| format!("The number of threads must be an integer, you entered '{}'", n))?;

        rayon::ThreadPoolBuilder::new().num_threads(n).build_global().unwrap();
    }
    // This unwrap does not cause a panic as "START" is a required arg, so Clap.rs throws error
    // if it is missing.
    let start = matches.value_of("START").unwrap();

    // Convert the input into a u64 integer. Error handling by Anyhow crate.
    let start = start.parse::<u64>()
                .with_context(|| format!("Start must be an integer, you entered '{}'", start))?;

    // handle parsing of stop same is start
    let stop = matches.value_of("STOP").unwrap();
    let stop = stop.parse::<u64>()
                .with_context(|| format!("Stop must be an integer, you entered '{}'", stop))?;


    // I believe the goal was to just print to stdout, so return the primes.
    // primes(start, stop)?;
    println!("{:?}", primes(start, stop)?);
    Ok(())
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
}
