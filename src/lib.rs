use algorithms::{PrngAlgorithm, MT19937_64};

mod algorithms;

pub struct Prng<T>
where
    T: PrngAlgorithm,
{
    seed: u64,
    prng: T,
}

/*
impl<T> Prng<T> {
    pub fn new(seed: u64) -> Self {
        let prng = T::from_seed(seed);
        Self { seed, prng }
    }
    
    pub fn uniform01(&mut self) -> f64 {
        self.uniform_open01();
    }
    
    pub fn reset(&mut self, seed: u64) {
    }
    
    pub fn seed() -> usize {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn rand() {
    }
}
*/
