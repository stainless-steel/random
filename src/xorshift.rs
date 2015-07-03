use Source;

/// The Xorshift+ algorithm.
///
/// ## References
///
/// 1. Sebastiano Vigna, “Further Scramblings of Marsaglia’s Xorshift
///    Generators,” CoRR, 2014.
///
/// 2. https://en.wikipedia.org/wiki/Xorshift
pub struct XorshiftPlus {
    state: [u64; 2],
}

impl XorshiftPlus {
    /// Create an instance of the algorithm.
    ///
    /// The seed should not be zero everywhere.
    #[inline(always)]
    pub fn new(seed: [u64; 2]) -> XorshiftPlus {
        debug_assert!(seed[0] | seed[1] != 0, "the seed should not be zero everywhere");
        XorshiftPlus { state: seed }
    }
}

impl Source for XorshiftPlus {
    #[inline(always)]
    fn read(&mut self) -> u64 {
        let (mut x, y) = (self.state[0], self.state[1]);

        self.state[0] = y;
        x = x ^ (x << 23);
        x = x ^ (x >> 17);
        x = x ^ y ^ (y >> 26);
        self.state[1] = x;

        x.wrapping_add(y)
    }
}

#[cfg(test)]
mod tests {
    use super::XorshiftPlus;

    #[test]
    #[should_panic]
    fn new_zero_seed() {
        let _ = XorshiftPlus::new([0, 0]);
    }
}
