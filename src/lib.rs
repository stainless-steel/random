//! Means of generating random numbers.

#[cfg(test)]
extern crate assert;

pub mod generators;
pub mod sources;

/// A random-number generator.
pub trait Generator {
    /// Draw a sample from the standard Gaussian distribution.
    fn gaussian(&mut self) -> f64;

    /// Draw a sample from the standard uniform distribution.
    fn uniform(&mut self) -> f64;
}

/// A source of randomness.
pub trait Source {
    /// Read the next chunk.
    fn next(&mut self) -> u64;
}
