pub fn nth(n: u32) -> Option<u32> {
    let mut primes_seen = 0;
    let mut i = 1;
    while primes_seen < n {
        i += 1;
        if is_prime(i) {
            primes_seen += 1;
            if primes_seen == n {
                return Some(i)
            }
        }
    }
    return None
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false
    } else if n <= 3 {
        return true
    } else if (n % 2 == 0 || n % 3 == 0) {
        return false
    } else {
        let mut p = 5;
        while p * p <= n {
            if n % p == 0 || n % (p + 2) == 0 {
                return false 
            }
            p += 6
        }
        return true
    }
}

/*
   function is_prime(n)
     if n ≤ 1
        return false
     else if n ≤ 3
        return true
     else if n mod 2 = 0 or n mod 3 = 0
        return false
     let i ← 5
     while i * i ≤ n
        if n mod i = 0 or n mod (i + 2) = 0
            return false
        i ← i + 6
     return true
 */

