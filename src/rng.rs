/// xorshift64star Pseudo-Random Number Generator
/// This class is based on original code written and dedicated
/// to the public domain by Sebastiano Vigna (2014).
/// It has the following characteristics:
///
///  -  Outputs 64-bit numbers
///  -  Passes Dieharder and SmallCrush test batteries
///  -  Does not require warm-up, no zeroland to escape
///  -  Internal state is a single 64-bit integer
///  -  Period is 2^64 - 1
///  -  Speed: 1.60 ns/call (Core i7 @3.40GHz)
///
/// For further analysis see
///   <http://vigna.di.unimi.it/ftp/papers/xorshift.pdf>

pub struct Prng {
  // Seed
  seed: u64,
}

impl Prng {
    pub fn new(seed: u64) -> Prng {
        Prng { seed, }
    }

    fn rand64(&mut self) -> u64 {
        self.seed ^= self.seed >> 12;
        self.seed ^= self.seed << 25;
        self.seed ^= self.seed >> 27;
        u64::wrapping_mul(self.seed, 2685821657736338717)
    } 

    fn sparse_rand64(&mut self) -> u64 {
        self.rand64() & self.rand64() & self.rand64()
    } 

    pub fn rand<T>(&mut self) -> T
        where T: From<u64> 
    {
        T::from(self.rand64())
    }

    pub fn sparse_rand<T>(&mut self) -> T
        where T: From<u64> 
    {
        T::from(self.sparse_rand64())
    }
}

