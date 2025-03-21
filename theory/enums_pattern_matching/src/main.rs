use std::collections::HashMap;

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut primes = HashMap::new();

    // Initialize the HashMap with true for all numbers from 2 to limit
    for i in 2..=limit {
        primes.insert(i, true);
    }

    // Perform the sieve
    for i in 2..=((limit as f64).sqrt() as usize) {
        if *primes.get(&i).unwrap() {
            let mut multiple = i * i;
            while multiple <= limit {
                primes.insert(multiple, false);
                multiple += i;
            }
        }
    }

    // Collect all numbers marked as true (prime)
    primes
        .into_iter()
        .filter(|&(_, is_prime)| is_prime)
        .map(|(num, _)| num)
        .collect()
}

fn main() {
    let limit = 50; // Change this to any upper limit you want
    let primes = sieve_of_eratosthenes(limit);

    println!("Prime numbers up to {}: {:?}", limit, primes);
}