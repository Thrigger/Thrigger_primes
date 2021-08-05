#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//pub fn get_next_prime(current: i64) -> i64 {
//
//    let mut next = current + 1;
//    while !is_prime(next) {
//        next += 1;
//    }
//    next
//}

fn is_prime_brute_force(start: i64, numb: i64) -> bool {
    let mut i = start;

    while i*i < numb {
        if numb % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

pub fn is_prime(numb: i64) -> bool {
    let first_primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
    
    if numb <= 19 { 
        /* Check if it is one of the first primes.
         * If it is one of the first primes then return true */
        for prime in first_primes {
            if prime == numb {
                return true;
            }
        }        

        /* check speciall case where numb is 1 */
        if numb == 1 {
            return true;
        }

        /* Return false since it is not a prime. */
        return false;
    } else {
        /* If the number is over 19 then check if it is dividable by any of the first primes.
         * if it is dividable then return false */
        for prime in first_primes {
            if numb % prime == 0 {
                return false;
            }
        }

        /* If it might be a prime then do a brute force check */ 
        return is_prime_brute_force(23, numb);
    }
}

