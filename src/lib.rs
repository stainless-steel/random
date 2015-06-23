//! Sources of randomness.

/// A source of randomness.
pub trait Generator {
    /// Read the next chunk.
    fn read(&mut self) -> u64;

    /// Read the next quantity.
    #[inline(always)]
    fn next<T: Quantity>(&mut self) -> T {
        Quantity::make(self.read())
    }
}

/// A random quantity.
pub trait Quantity {
    /// Make up a random quantity.
    fn make(u64) -> Self;
}

impl Quantity for f64 {
    #[inline(always)]
    fn make(chunk: u64) -> f64 {
        chunk as f64 / (std::u64::MAX as f64 + 1.0)
    }
}

impl Quantity for u64 {
    #[inline(always)]
    fn make(chunk: u64) -> u64 {
        chunk
    }
}

mod xorshift;

pub use xorshift::XorshiftPlus;

#[cfg(test)]
mod tests {
    use {Generator, XorshiftPlus};

    #[test]
    fn read() {
        let mut generator = XorshiftPlus::new([42, 42]);
        let _ = generator.next::<f64>();
        let _ = generator.next::<u64>();
    }
}
