fn primes(max_len: usize) -> Vec<usize> {
    // Sieve of Eratosthenes 
    let mut can = vec![1; max_len];  // 1 is prime
    for i in 2..max_len {
        // If i is prime
        if can[i] == 1 {
            // Set multiples of i to not prime
            for step in 2..((max_len / i) + 1) {
                can[(step * i) - 1] = 0;
            }
        }
    }
    let mut primes = Vec::new();
    for i in 1..max_len { 
        if can[i] == 1 {
            primes.push(i + 1);
        }
    }
    return primes.to_vec();
}

fn main() {
    let p = primes(525600);
    println!("n primes: {:?}", p.len());
}
