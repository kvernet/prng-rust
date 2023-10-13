/* Period parameters */
const N: usize = 624;
const M: usize = 397;
const MATRIX_A: u64 = 0x9908b0df;    /* constant vector a */
const UPPER_MASK: u64 = 0x80000000;  /* most significant w-r bits */
const LOWER_MASK: u64 = 0x7fffffff;  /* least significant r bits */
const Y_MAX: u64 = 0xffffffff;

/* Tempering parameters */
const TEMPERING_MASK_B: u64 = 0x9d2c5680;
const TEMPERING_MASK_C: u64 = 0xefc60000;

const MAG01: [u64; 2] = [0x0, MATRIX_A];

fn tempering_shift_u(y: u64) -> u64 { y >> 11 }
fn tempering_shift_s(y: u64) -> u64 { y << 7 }
fn tempering_shift_t(y: u64) -> u64 { y << 15 }
fn tempering_shift_l(y: u64) -> u64 { y >> 18 }


struct Prng {
    seed: u64,
    mt: [u64;N],
    index: usize
}

#[allow(dead_code, unused_variables)]
impl Prng {
    pub fn new(seed: u64) -> Self {
        let mt = [0; N];
        let index = 0;
        let mut obj = Self {seed, mt, index};
        obj.reset(seed);
        obj
    }
    
    pub fn uniform01(&mut self) -> f64 {
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
        
        /* return y; */ /* for integer generation */
        (y as f64) / (Y_MAX as f64)
    }
    
    pub fn reset(&mut self, seed: u64) {
        self.seed = seed;        
        self.mt[0] = seed & 0xffffffff;        
        for i in 1..N {
            self.mt[i] = (69069 * self.mt[i-1]) & 0xffffffff;
        }        
        self.index = N - 1;
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn unform01() {
        let mut prng = Prng::new(1345);
        
        for _i in 1..1000 {
            let u = prng.uniform01();
            assert!(u >= 0.0 && u <= 1.0);
        }
    }
}
