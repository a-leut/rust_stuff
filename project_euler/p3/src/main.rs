fn primes(max_len: usize) -> Vec<usize> {
    // Sieve of Eratosthenes 
    let mut candidate = vec![1; max_len];  // 1 is prime
    for i in 2..max_len {
        // If i is prime
        if candidate[i] == 1 {
            let mut step = 2;
            let mut index = step * i;
            while index < max_len {
                candidate[index] = 0;
                step += 1;
                index = step * i;
            }
        }
    }
    let mut primes = Vec::new();
    for i in 2..max_len { 
        if candidate[i] == 1 {
            primes.push(i);
        }
    }
    return primes.to_vec();
}

fn main() {
    let p = primes(100);
    println!("n primes: {:?}", p);
}
