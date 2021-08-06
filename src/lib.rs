#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_prime_brute_force() {
        /* Some basic sanity tests */
        assert_eq!(is_prime_brute_force(3, 17), true);
        assert_eq!(is_prime_brute_force(3, 18), false);
        assert_eq!(is_prime_brute_force(3, 19), true);
        assert_eq!(is_prime_brute_force(3, 29), true);
    }

    #[test]
    fn test_is_prime() {
        /* Some basic sanity tests */
        assert_eq!(is_prime(5), true);
        assert_eq!(is_prime(7), true);
        assert_eq!(is_prime(17), true);
        assert_eq!(is_prime(18), false);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(29), true);
    }

    #[test]
    fn test_get_next() {
        /* Some basic sanity tests */
        assert_eq!(get_next(5), 7);
        assert_eq!(get_next(7), 11);
        assert_eq!(get_next(17), 19);
        assert_eq!(get_next(18), 19);
        assert_eq!(get_next(19), 23);
        assert_eq!(get_next(29), 31);
    }

}

pub fn get_next(current: i64) -> i64 {

    let mut next = match current % 2 {
        0 => current + 1,
        _ => current + 2
    };

    while !is_prime(next) {
        next += 2;
    }
    next
}

fn is_prime_brute_force(start: i64, numb: i64) -> bool {
    let mut i = start;

    while i < numb {
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

