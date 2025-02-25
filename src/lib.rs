pub struct Solution;

impl Solution {
    /// Computes n^p mod m efficiently using exponentiation by squaring.
    #[inline]
    fn modpow(mut n: i32, mut p: i32, m: i32) -> i32 {
        let mut res = 1;
        n %= m;
        while p > 0 {
            if p & 1 == 1 {
                res = (res * n) % m;
            }
            n = (n * n) % m;
            p >>= 1;
        }
        res
    }

    /// Decomposes x into (y, ct) where x = y * m^ct, y not divisible by m.
    #[inline]
    fn decompose(mut x: i32, m: i32) -> (i32, i32) {
        let mut ct = 0;
        while x % m == 0 {
            x /= m;
            ct += 1;
        }
        (x % m, ct)
    }

    /// Checks if sum of binomial-weighted differences is 0 mod m.
    fn check(s: &str, m: i32) -> bool {
        let n = s.len() as i32;
        if n <= 1 {
            return true;
        }
        
        let mut res = 0;
        let mut ncr = 1;
        let mut ct = 0;
        
        // Use iterator to avoid Vec allocation
        let bytes = s.as_bytes();
        for i in 0..n - 1 {
            // Compute difference directly from bytes
            let diff = (bytes[i as usize] as i32 - b'0' as i32) - 
                       (bytes[i as usize + 1] as i32 - b'0' as i32);
            let ai = (diff + 10) % 10;
            
            if ct == 0 {
                res = (res + ai * ncr) % m;
            }
            
            if i < n - 2 { // Last iteration doesn’t update ncr
                let (p1_y, p1_ct) = Self::decompose(n - 2 - i, m);
                ncr = (ncr * p1_y) % m;
                ct += p1_ct;
                
                let (p2_y, p2_ct) = Self::decompose(i + 1, m);
                // Precompute modular inverse only if ct allows contribution
                ncr = (ncr * Self::modpow(p2_y, m - 2, m)) % m;
                ct -= p2_ct;
            }
        }
        
        res == 0
    }

    /// Main function optimized to avoid allocations and redundant conversions.
    pub fn has_same_digits(s: String) -> bool {
        // Use &str to avoid ownership transfer if possible
        let s = s.as_str();
        Self::check(s, 2) && Self::check(s, 5)
    }
}