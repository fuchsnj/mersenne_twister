use std::num::Wrapping;

const MAGIC: [Wrapping<u64>; 2] = [Wrapping(0), Wrapping(0xB5026F5AA96619E9)];
const W: usize = 64;
const N: usize = 312;
const M: usize = 156;
const DIFF: isize = M as isize - N as isize;
const R: u64 = 31;
const A: Wrapping<u64> = Wrapping(0xB5026F5AA96619E9);
const U: usize = 29;
const D: Wrapping<u64> = Wrapping(0x5555555555555555);
const S: usize = 17;
const B: Wrapping<u64> = Wrapping(0x71D67FFFEDA60000);
const T: usize = 37;
const C: Wrapping<u64> = Wrapping(0xFFF7EEE000000000);
const L: usize = 43;
const F: Wrapping<u64> = Wrapping(6364136223846793005);
const DEFAULT_SEED: u64 = 5489;

const UPPER_MASK: Wrapping<u64> = Wrapping(0xFFFFFFFF80000000);
const LOWER_MASK: Wrapping<u64> = Wrapping(0x7FFFFFFF);

pub struct MersenneTwister64 {
    state: [Wrapping<u64>; N],
    index: usize,

}

impl MersenneTwister64 {
    fn new_unseeded() -> MersenneTwister64 {
        MersenneTwister64 {
            state: [Wrapping(0); N],
            index: N + 1,
        }
    }
    pub fn new() -> MersenneTwister64 {
        let mut output = MersenneTwister64::new_unseeded();
        output.seed(DEFAULT_SEED);
        output
    }

    pub fn new_from_array_seed(seed: &[u64]) -> MersenneTwister64 {
        let mut output = MersenneTwister64::new_unseeded();
        output.seed_by_array(seed);
        output
    }

    fn seed(&mut self, seed: u64) {
        self.state[0] = Wrapping(seed);

        for i in 1..N {
            let prev = self.state[i - 1];
            self.state[i] = F * (prev ^ (prev >> (W - 2))) + Wrapping(i as u64);
        }
        self.index = N;
    }

    fn seed_by_array(&mut self, seed: &[u64]) {
        self.seed(19650218);
        let mut i = 1;
        let mut j = 0;
        for _ in 0..N.max(seed.len()) {
            self.state[i] = (self.state[i] ^ ((self.state[i - 1] ^ (self.state[i - 1] >> 62))
                * Wrapping(3935559000370003845))) + Wrapping(seed[j]) + Wrapping(j as u64);
            i += 1;
            if i >= N {
                self.state[0] = self.state[N - 1];
                i = 1;
            }
            j += 1;
            if j >= seed.len() {
                j = 0;
            }
        }
        for _ in 0..N - 1 {
            self.state[i] = (self.state[i] ^ ((self.state[i - 1] ^ (self.state[i - 1] >> 62))
                * Wrapping(2862933555777941757))) - Wrapping(i as u64);
            i += 1;
            if i >= N {
                self.state[0] = self.state[N - 1];
                i = 1;
            }
        }
        self.state[0] = Wrapping(1 << 63);
    }

    pub fn next_u64(&mut self) -> u64 {
        if self.index >= N {
            self.twist();
        }
        let mut y = self.state[self.index];
        y ^= (y >> U) & D;
        y ^= (y << S) & B;
        y ^= (y << T) & C;
        y ^= y >> L;
        self.index += 1;
        y.0
    }

    fn twist(&mut self) {
        for index in 0..(N - M) {
            let y = (self.state[index] & UPPER_MASK) | (self.state[index + 1] & LOWER_MASK);
            let magic_idx = (y.0 & 0x1) as usize;
            self.state[index] = self.state[index + M] ^ (y >> 1) ^ MAGIC[magic_idx];
        }

        for index in (N - M)..(N - 1) {
            let y = (self.state[index] & UPPER_MASK) | (self.state[index + 1] & LOWER_MASK);
            let magic_idx = (y.0 & 0x1) as usize;
            let nindex = index as isize + DIFF;
            self.state[index] = self.state[nindex as usize] ^ (y >> 1) ^ MAGIC[magic_idx];
        }

        let y = (self.state[N - 1] & UPPER_MASK) | (self.state[0] & LOWER_MASK);
        let magic_idx = (y.0 & 0x1) as usize;
        self.state[N - 1] = self.state[M - 1] ^ (y >> 1) ^ MAGIC[magic_idx];

        self.index = 0;
    }
}

#[cfg(test)]
mod test {
    use crate::MersenneTwister64;

    #[test]
    fn test_array_seed() {
        // reference values from http://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/mt19937-64.out.txt
        let mut twister = MersenneTwister64::new_from_array_seed(&[0x12345, 0x23456, 0x34567, 0x45678]);
        assert_eq!(twister.next_u64(), 7266447313870364031);
        assert_eq!(twister.next_u64(), 4946485549665804864);
    }

    #[test]
    fn test_default_seed() {
        // reference values from https://oeis.org/A221558
        let mut twister = MersenneTwister64::new();
        assert_eq!(twister.next_u64(), 14514284786278117030);
    }
}

