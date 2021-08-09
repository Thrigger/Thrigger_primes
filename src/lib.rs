/// Returns the next prime number.
///
/// The functions takes a number as a input and returns the next prime number after that.
/// This function is NOT optimized for speed and shall not be used if speed is needed.
pub fn get_next(current: u64) -> u64 {

    let mut next = match current % 2 {
        0 => current + 1,
        _ => current + 2
    };

    while !is_prime(next) {
        next += 2;
    }
    next
}

fn is_prime_brute_force(start: u64, numb: u64) -> bool {
    let mut i = start;

    while i < numb {
        if numb % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

pub fn is_prime(numb: u64) -> bool {
    let first_primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53];
    
    if numb <= 53 { 
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
        /* If the number is over 53 then check if it is dividable by any of the first primes.
         * if it is dividable then return false */
        for prime in first_primes {
            if numb % prime == 0 {
                return false;
            }
        }

        /* If it might be a prime then do a brute force check */ 
        return is_prime_brute_force(57, numb);
    }
}

pub fn get_primes(last: usize) -> Vec<u64> {
    /* A vector of booleans for each value up to limit. 
     * possible_primes[0] => is 2 a prime?
     * possible_primes[1] => is 3 a prime? 
     * and so on.... */
    let mut possible_primes: Vec<bool> = vec![true; last-1];
    let mut primes: Vec<u64> = vec![];

    let mut current_prime: u64 = 2;
    
    while current_prime as usize <= last {
        /* Add prime to output. */
        primes.push(current_prime);

        /* Remove multiples of this prime in list. */
        let mut i = 2*(current_prime as usize);
        while i <= last {
            possible_primes[i-2] = false;   
            i += current_prime as usize;
        }

        /* Find next prime. */
        loop {
            current_prime += 1;
            if current_prime as usize > last || 
                possible_primes[current_prime as usize-2] == true {
                break;
            }
        }
    }

    primes
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_primes() {
        /* Some basic sanity tests */
        assert_eq!(get_primes(20), [2, 3, 5, 7, 11, 13, 17, 19]);
    }

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


