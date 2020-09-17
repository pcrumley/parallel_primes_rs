use clap::{App, Arg};
use rayon::prelude::*;

fn is_prime(num: u64) -> bool {
    !(2..num).any(|x| num % x == 0)
}

fn primes(start: u64, stop: u64) -> Vec<u64> {
    (start..=stop).into_par_iter()
        .filter_map(|n| {
            if is_prime(n) { Some(n) }
            else { None }
        }).collect()
}

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let matches = App::new("Parallel Primes")
                          .version("1.0")
                          .author("Patrick C. <patrick.crumley@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("START")
                               .help("The number we start looking for primes")
                               .required(true)
                               .index(1))
                          .arg(Arg::with_name("STOP")
                                .help("Sets the input file to use")
                                .required(true)
                                .index(2))
                          .get_matches();
    let start = matches.value_of("START").unwrap().parse::<u64>().unwrap();
    let stop = matches.value_of("STOP").unwrap().parse::<u64>().unwrap();

    println!("{:?}", primes(start, stop));
}
