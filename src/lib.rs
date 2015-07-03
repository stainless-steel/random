//! Sources of randomness.
//!
//! ## Example
//!
//! ```
//! use random::Source;
//!
//! let mut source = random::default().seed([42, 69]);
//! let uniforms = source.iter().take(100).collect::<Vec<f64>>();
//! ```

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;

/// A source of randomness.
pub trait Source {
    /// Return the next raw chunk.
    fn read(&mut self) -> u64;

    /// Return the next quantity.
    #[inline(always)]
    fn take<T: Quantity>(&mut self) -> T where Self: Sized {
        Quantity::make(self.read())
    }

    /// Return a sequence of quantities.
    #[inline]
    fn iter<'l, T: Quantity>(&'l mut self) -> Sequence<'l, Self, T> where Self: Sized {
        Sequence { source: self, phantom: PhantomData }
    }
}

/// A sequence of random quantities.
pub struct Sequence<'l, S: ?Sized, Q> where S: Source + 'l, Q: Quantity + 'l {
    source: &'l mut S,
    phantom: PhantomData<&'l Q>,
}

/// A random quantity.
pub trait Quantity {
    /// Make up a quantity from a raw chunk.
    fn make(u64) -> Self;
}

impl<'l, S, Q> Iterator for Sequence<'l, S, Q> where S: Source, Q: Quantity {
    type Item = Q;

    #[inline(always)]
    fn next(&mut self) -> Option<Q> {
        Some(self.source.take())
    }
}

impl Quantity for f64 {
    #[inline(always)]
    fn make(chunk: u64) -> f64 {
        chunk as f64 / (::std::u64::MAX as f64 + 1.0)
    }
}

impl Quantity for u64 {
    #[inline(always)]
    fn make(chunk: u64) -> u64 {
        chunk
    }
}

/// The default source, which is the Xorshift128+ algorithm.
pub struct Default(Rc<RefCell<Xorshift128Plus>>);

impl Default {
    /// Seed the source.
    ///
    /// The seed should not be zero everywhere.
    #[inline(always)]
    pub fn seed(self, seed: [u64; 2]) -> Default {
        *self.0.borrow_mut() = Xorshift128Plus::new(seed);
        self
    }
}

impl Source for Default {
    #[inline(always)]
    fn read(&mut self) -> u64 {
        self.0.borrow_mut().read()
    }
}

/// Return the default source.
///
/// Each thread has its own copy of the source, and each copy is initialized
/// with the same default seed. Consequently, the usage is thread safe; however,
/// each thread is responsible for reseeding its source.
#[inline(always)]
pub fn default() -> Default {
    thread_local!(static DEFAULT: Rc<RefCell<Xorshift128Plus>> = {
        Rc::new(RefCell::new(Xorshift128Plus::new([42, 69])))
    });
    Default(DEFAULT.with(|source| source.clone()))
}

mod xorshift;
pub use self::xorshift::Xorshift128Plus;
