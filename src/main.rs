use clap::{App, Arg};
use rayon::prelude::*;
use anyhow::{Context, Result, ensure};

fn is_prime(num: u64) -> bool {
    !(2..num).any(|x| num % x == 0)
}

fn primes(start: u64, stop: u64) -> Vec<u64> {
    // There 
    // there are no primes less than 2 so need to
    // start there.
    let start = start.max(2);
    (start..=stop).into_par_iter()
        .filter_map(|n| {
            if is_prime(n) { Some(n) }
            else { None }
        }).collect()
}

fn main() -> Result<()> {
    // Use the Clap.rs argument parser to capture user inputs. Does nice job of adding
    // a help flag & throws returns an error if required args are missing.
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
    println!("{:?}", primes(start, stop));
    Ok(())
}
