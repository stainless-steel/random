//! Sources of randomness.
//!
//! ## Example
//!
//! ```
//! use random::Source;
//!
//! let mut source = random::default().seed([42, 69]);
//! println!("Scalar: {:?}", source.read::<f64>());
//! println!("Vector: {:?}", source.iter().take(2).collect::<Vec<f64>>());
//! ```

mod default;
mod sequence;
mod source;
mod value;
mod xorshift;

pub use default::Default;
pub use sequence::Sequence;
pub use source::Source;
pub use value::Value;
pub use xorshift::Xorshift128Plus;

/// Create an instance of the default source.
///
/// For further information, see `Default::new`.
#[inline(always)]
pub fn default() -> Default {
    Default::new()
}
