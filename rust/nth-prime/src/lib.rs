pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")
    let mut primes = vec![];
    'all_numbers: for num in 2.. {
        for prime in primes.iter() {
            if num % prime == 0 {
                continue 'all_numbers;
            }
        }
        if primes.len() as u32 == n {
            return num;
        }
        primes.push(num);
    }
    unimplemented!("prime number was larger than fittable inside u32")
}
