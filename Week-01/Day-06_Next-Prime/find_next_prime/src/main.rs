// https://docs.rs/primes/latest/primes/
use primes::{Sieve, PrimeSet};

fn next_prime(integer: u64) -> u64 {
    
    let mut pset = Sieve::new();
    let (_, n) = pset.find(integer);
    println!("Next prime: {}", n);
    n
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(next_prime(12), 13);
    }
    #[test]
    fn example_2() {
        assert_eq!(next_prime(24), 29);
    }
    #[test]
    fn example_3() {
        assert_eq!(next_prime(11), 11);
    }
}