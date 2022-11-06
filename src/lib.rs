//! Sources of randomness.
//!
//! ## Example
//!
//! ```
//! use random::Source;
//!
//! let mut source = random::default(42);
//! println!("Scalar: {:?}", source.read::<f64>());
//! println!("Vector: {:?}", source.iter().take(2).collect::<Vec<f64>>());
//! ```

#![no_std]

mod sequence;
mod source;
mod value;
mod xorshift;

pub use sequence::Sequence;
pub use source::Source;
pub use value::Value;
pub use xorshift::Xorshift128Plus;

pub use Xorshift128Plus as Default;

/// Create an instance of the default source.
///
/// The default source is the Xorshift128+ algorithm.
#[inline(always)]
pub fn default(seed: u64) -> Default {
    Xorshift128Plus::new([seed.wrapping_sub(1), seed.wrapping_add(1)])
}
