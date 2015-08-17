extern crate Euler;

fn main() {
    let primes = Euler::esieve((80 * 80) + (1000 * 80) + (1000));

    let is_prime = |n: usize| {
        match primes.binary_search_by(|elem| elem.cmp(&n)) {
            Ok(_) => true,
            Err(_) => false,
        }
    };

    let mut best = (0, 0, 0);
/*

    for a in (-999..1000) {
        for b in (-999..1000) {
            let maybe_prime = |n: usize| n*n + a*n + b;
            for n in (0..) {
                if !is_prime(maybe_prime(n)) {
                    break;
                } else if n > best.2 {
                    best = (a, b, n);
                }
            }
        }
    }
*/
/*
    let mut maybeprime = 2;

    for a in (-999..1000) {
        println!("debug");
        for b in (-999..1000) {
            for n in (0..) {
                maybeprime = n*n + a*n + b;
                if !is_prime(maybeprime) {
                    break;
                } else if n > best.2 {
                    best = (a, b, n);
                }
            }
        }
    }
*/


    let mut acum = 0;

    let mut maybeprime = 2;

 
    println!("{:?} ", maybeprime);

    for a in (-999..1000) {
        for b in (-999..1000) {
            for n in (0..) {
                maybeprime =  n*n + a*n + b;
                acum += 1;
                //let isprim = testprime(&primes, n*n + a*n + b);
                let m = n*n + a*n + b;
                //let p = testfunc(&primes, m);
                if n > 2 {
                    break;
                }

            }

        }
    }

    let mut isprime = false;
    for p in primes.iter() {
        if *p == maybeprime {
            isprime = true;
            break;
        }
        if *p < maybeprime {
            isprime = false;
            break;
        }
    }


    println!("{:?}", acum);


    println!("{:?}", best);
}

fn testprime(primelist: &Vec<usize>, num: usize) -> bool {
    /*match primelist.binary_search_by(|prime| prime.cmp(&num)) {
        Ok(_) => true,
        Err(_) => false,
    }*/
    true
}

fn testfunc(primelist: &Vec<usize>,  num: usize) -> bool {
    true
}