use rand::prelude::*;

fn main() {
    // create a pseudo-random number generatr for the current thread
    let mut rng = thread_rng();

    // print an integer number
    // between 0 (included) and 20 (excluded)
    println!("{}", rng.gen_range(0..20));

    // floating point number
    println!("{}", rng.gen::<f64>());

    // generate a Bool
    println!("{}", if rng.gen() { "heads" } else { "tails" });
}
