use Source;

/// The Xorshift128+ algorithm.
///
/// ## References
///
/// 1. Sebastiano Vigna, “Further Scramblings of Marsaglia’s Xorshift
///    Generators,” CoRR, 2014.
///
/// 2. https://en.wikipedia.org/wiki/Xorshift#Xorshift.2B
pub struct Xorshift128Plus {
    state: [u64; 2],
}

impl Xorshift128Plus {
    /// Create an instance of the algorithm.
    ///
    /// The seed should not be zero everywhere.
    #[inline(always)]
    pub fn new(seed: [u64; 2]) -> Xorshift128Plus {
        debug_assert!(seed[0] | seed[1] != 0, "the seed should not be zero everywhere");
        Xorshift128Plus { state: seed }
    }
}

impl Source for Xorshift128Plus {
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
    use super::Xorshift128Plus;

    #[test]
    #[should_panic]
    fn new_zero_seed() {
        let _ = Xorshift128Plus::new([0, 0]);
    }
}
