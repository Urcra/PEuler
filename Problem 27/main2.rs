extern crate Euler;

fn main() {
    let primes = ((80 * 80) + (1000 * 80) + (1000));

    let mut best = (0, 0, 0);


    let mut maybeprime = 2;

    let mut acum = 0;
 
    println!("{:?} ", maybeprime);


    for i in -999..1000 {
        acum += 1;
        for j in -999..1000 {
            let res = primes.binary_search_by(|elem| elem.cmp(&(i*6)));
        }
    }

/*
    for a in (-999..1000) {
        for b in (-999..1000) {
            for n in (0..) {
                acum+= 1;
               maybeprime =  n*n + a*n + b;

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

                if n > 2 {
                    break;
                }

            }

        }
    }
    */
/*
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

*/
    println!("{:?}", acum);


    println!("{:?}", best);
}
