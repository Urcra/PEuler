

//Sieve of Eratosthenes
pub fn esieve(limit: usize) -> Vec<usize> {
    let mut primes: Vec<usize> = Vec::new();
    let sqrtmax = (limit as f64).sqrt();
    let mut eliminated: Vec<bool> = vec![false; limit+1];

    primes.push(2);
/*
    Step_by is unstable ;((((

    for i in (3..limit).step_by(2) {
        if !eliminated[i] {
            if (i as f64) < sqrtmax {
                for j in ((i*i)..limit).step_by(2*i) {
                    eliminated[j] = true;
                }
            }
            primes.push(i);
        }
    }
*/
    // Initiate poor mans for loop.... RIP performance

    let mut i = 3;
    while i < limit {
        if !eliminated[i] {
            if (i as f64) < sqrtmax {
                let mut j = i * i;
                while j < limit { 
                    eliminated[j] = true;
                    j += 2 * i;
                }
            }
            primes.push(i);
        }
        i += 2;
    }
    primes
}