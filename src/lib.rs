//! Sources of randomness.

/// A random quantity.
pub trait Quantity {
    /// Make up a random quantity.
    fn make(u64) -> Self;
}

/// A source of randomness.
pub trait Source {
    /// Read the next chunk.
    fn read(&mut self) -> u64;

    /// Read the next quantity.
    #[inline(always)]
    fn next<T>(&mut self) -> T where T: Quantity {
        Quantity::make(self.read())
    }
}

impl Quantity for f64 {
    #[inline(always)]
    fn make(chunk: u64) -> f64 {
        chunk as f64 / (std::f64::MAX as f64 + 1.0)
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
    use {Source, XorshiftPlus};

    #[test]
    fn read() {
        let mut source = XorshiftPlus::new([42, 42]);
        let _ = source.next::<f64>();
        let _ = source.next::<u64>();
    }
}
