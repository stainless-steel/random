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
use std::marker::PhantomData;
use std::rc::Rc;

/// A source of randomness.
pub trait Source {
    /// Read a random `u64`.
    ///
    /// The implied distribution is a uniform distribution whose support is the
    /// set `{0, 1, …, u64::MAX}`.
    fn read_u64(&mut self) -> u64;

    /// Read a random `f64`.
    ///
    /// The implied distribution is a uniform distribution whose support is the
    /// interval `[0, 1]`.
    #[inline(always)]
    fn read_f64(&mut self) -> f64 {
        self.read_u64() as f64 / (::std::u64::MAX as f64 + 1.0)
    }

    /// Read a random element.
    #[inline(always)]
    fn read<T: Element>(&mut self) -> T where Self: Sized {
        Element::from(self)
    }

    /// Read a sequence of random elements.
    #[inline(always)]
    fn iter<'l, T: Element>(&'l mut self) -> Sequence<'l, Self, T> where Self: Sized {
        Sequence { source: self, phantom: PhantomData }
    }
}

/// A sequence of random elements.
pub struct Sequence<'l, S: ?Sized, E> where S: Source + 'l, E: Element + 'l {
    source: &'l mut S,
    phantom: PhantomData<&'l E>,
}

/// A random element.
pub trait Element {
    /// Create a random element from a source.
    fn from<T: Source>(&mut T) -> Self;
}

impl<'l, S, E> Iterator for Sequence<'l, S, E> where S: Source, E: Element {
    type Item = E;

    #[inline(always)]
    fn next(&mut self) -> Option<E> {
        Some(self.source.read())
    }
}

impl Element for f64 {
    #[inline(always)]
    fn from<T: Source>(source: &mut T) -> f64 {
        source.read_f64()
    }
}

impl Element for u64 {
    #[inline(always)]
    fn from<T: Source>(source: &mut T) -> u64 {
        source.read_u64()
    }
}

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

mod xorshift;
pub use self::xorshift::Xorshift128Plus;
