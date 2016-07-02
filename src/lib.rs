//! Sources of randomness.
//!
//! ## Example
//!
//! ```
//! use random::Source;
//!
//! let mut source = random::default().seed([42, 69]);
//! let one = source.read::<f64>();
//! let two = source.iter().take(2).collect::<Vec<f64>>();
//! ```

use std::cell::RefCell;
use std::rc::Rc;

/// The default source, which is the Xorshift128+ algorithm.
#[derive(Clone)]
pub struct Default(Rc<RefCell<Xorshift128Plus>>);

impl Default {
    /// Seed the source.
    ///
    /// At least one bit of the seed should be nonzero.
    #[inline(always)]
    pub fn seed(self, seed: [u64; 2]) -> Default {
        *self.0.borrow_mut() = Xorshift128Plus::new(seed);
        self
    }
}

impl Source for Default {
    #[inline(always)]
    fn read_u64(&mut self) -> u64 {
        self.0.borrow_mut().read_u64()
    }
}

/// Return the default source.
///
/// Each thread has its own copy of the source, and each copy is initialized
/// with the same default seed. Consequently, the usage is thread safe; however,
/// each thread is responsible for reseeding its source.
pub fn default() -> Default {
    thread_local!(static DEFAULT: Rc<RefCell<Xorshift128Plus>> = {
        Rc::new(RefCell::new(Xorshift128Plus::new([42, 69])))
    });
    Default(DEFAULT.with(|source| source.clone()))
}

mod sequence;
mod source;
mod value;
mod xorshift;

pub use sequence::Sequence;
pub use source::Source;
pub use value::Value;
pub use xorshift::Xorshift128Plus;
