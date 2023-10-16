/* Period parameters */
const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u64 = 0x9908b0df;    /* constant vector a */
const UPPER_MASK: u64 = 0x80000000;  /* most significant w-r bits */
const LOWER_MASK: u64 = 0x7fffffff;  /* least significant r bits */

/* Tempering parameters */
const TEMPERING_MASK_B: u64 = 0x9d2c5680;
const TEMPERING_MASK_C: u64 = 0xefc60000;

const MAG01: [u64; 2] = [0x0, MATRIX_A];

fn tempering_shift_u(y: u64) -> u64 { y >> 11 }
fn tempering_shift_s(y: u64) -> u64 { y << 7 }
fn tempering_shift_t(y: u64) -> u64 { y << 15 }
fn tempering_shift_l(y: u64) -> u64 { y >> 18 }


pub struct Prng {
    seed: u64,
    mt: [u64;N],
    index: usize
}

impl Prng {
    pub fn new(seed: u64) -> Self {
        let mt = [0; N];
        let index = 0;
        let mut obj = Self {seed, mt, index};
        obj.reset(seed);
        obj
    }
    
    pub fn rand(&mut self) -> u64 {
        let mut y: u64;
        if self.index >= N {
            let mut _kk: usize;
            
            let mut slice: usize;
            for kk in 0..N-M {
                y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk+1] & LOWER_MASK);
                slice = y as usize & 0x1;
                self.mt[kk] = self.mt[kk+M] ^ (y >> 1) ^ MAG01[slice];
            }
            for kk in N-M..N-1 {
                y = (self.mt[kk] & UPPER_MASK) | (self.mt[kk+1] & LOWER_MASK);
                slice = y as usize & 0x1;
                self.mt[kk] = self.mt[kk+M-N] ^ (y >> 1) ^ MAG01[slice];
            }
            y = (self.mt[N-1] & UPPER_MASK) | (self.mt[0] & LOWER_MASK);
            slice = y as usize & 0x1;
            self.mt[N-1] = self.mt[M-1] ^ (y >> 1) ^ MAG01[slice];
            
            self.index = 0;
        }
        
        y = self.mt[self.index];
        self.index += 1;
        
        y ^= tempering_shift_u(y);
        y ^= tempering_shift_s(y) & TEMPERING_MASK_B;
        y ^= tempering_shift_t(y) & TEMPERING_MASK_C;
        y ^= tempering_shift_l(y);
        y
    }
    
    pub fn uniform01(&mut self) -> f64 {
        return ((self.rand() as f64) + 0.5)*(1.0/4294967296.0); 
    }
    
    pub fn reset(&mut self, seed: u64) {
        self.seed = seed;        
        self.mt[0] = seed & 0xffffffff_u64;        
        for i in 1..N {
            self.mt[i] = (69069 * self.mt[i-1]) & 0xffffffff;
        }        
        self.index = N + 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_array(init_key: &[u64]) -> Prng {
        let key_length = init_key.len();
        let mut prng = Prng::new(19650218_u64);
        let mut i: usize = 1;
        let mut j: usize = 0;
        let mut k: usize = if N > key_length { N } else { key_length };
        
        let mt = &mut prng.mt;
        while k > 0 {
            mt[i] = (mt[i] ^ ((mt[i-1] ^ (mt[i-1] >> 30)) * 1664525_u64))
                + init_key[j] + (j as u64); /* non linear */
            mt[i] &= 0xffffffff_u64; /* for WORDSIZE > 32 machines */
            i += 1;
            j += 1;
            if i >= N { mt[0] = mt[N-1]; i=1; }
            if j >= key_length { j = 0; }
            k -= 1;
        }
        k = N - 1;
        while k > 0 {
            mt[i] = (mt[i] ^ ((mt[i-1] ^ (mt[i-1] >> 30)) * 1566083941_u64))
                - i as u64; /* non linear */
            mt[i] &= 0xffffffff_u64; /* for WORDSIZE > 32 machines */
            i += 1;
            if i >= N { mt[0] = mt[N-1]; i=1; }
            k -= 1;
        }

        mt[0] = 0x80000000_u64; /* MSB is 1; assuring non-zero initial array */
        prng
    }
   
    #[test]
    fn rand() {
        let mut prng = from_array(&[0x123_u64, 0x234_u64, 0x345_u64, 0x456_u64]);
        const EXPECTED: [u64; 20] = [
            1067595299,  955945823,  477289528, 4107218783, 4228976476, 
            3344332714, 3355579695,  227628506,  810200273, 2591290167, 
            2560260675, 3242736208,  646746669, 1479517882, 4245472273, 
            1143372638, 3863670494, 3221021970, 1773610557, 1138697238,
        ];
        for i in 0..EXPECTED.len() {
            let u = prng.rand();
            assert_eq!(u, EXPECTED[i]);
        }
    }
}
