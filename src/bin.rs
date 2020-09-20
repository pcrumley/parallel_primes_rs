use anyhow::{Result, Context};
use clap::{App, Arg};
use primes_lib::primes;
struct Config {
    start: u64,
    stop: u64,
    num_threads: Option<usize>  ,
}

impl Config {
    fn new(matches: &clap::ArgMatches) -> Result<Config> {
        // see if the user wants to override the number of threads.
        let mut num_threads = None;
        if let Some(n) = matches.value_of("THREADS") {
            let n = n.parse::<usize>()
                .with_context(|| format!(
                "The number of threads must be an integer, you entered '{}'", n))?;
            num_threads = Some(n);
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
        Ok(Config {
            start,
            stop,
            num_threads,
        })
    }
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
                               .help("Sets the number of threads. If not supplied it will set the
number of threads equal to the number of cpu cores.")
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

    // do additional logic parsing, handling errors to user inputs to create a config
    let config = Config::new(&matches)?;
    // if the user specified a certain number of threads we set it here, otherwise
    // the number of threads is set to the number of cores.
    if let Some(n) = config.num_threads {
        rayon::ThreadPoolBuilder::new().num_threads(n).build_global().unwrap();
    }
    println!("{:?}", primes(config.start, config.stop)?);
    Ok(())
}
