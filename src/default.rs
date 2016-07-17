use std::cell::RefCell;
use std::rc::Rc;

use {Source, Xorshift128Plus};

/// An instance of the default source.
///
/// The default source is the Xorshift128+ algorithm.
#[derive(Clone)]
pub struct Default(Rc<RefCell<Xorshift128Plus>>);

impl Default {
    /// Create an instance of the default source.
    ///
    /// Each thread has its own copy of the default source, and each copy is
    /// initialized with the same default seed. Therefore, the usage is thread
    /// safe; however, each thread is responsible for reseeding its source.
    pub fn new() -> Default {
        thread_local!(static DEFAULT: Rc<RefCell<Xorshift128Plus>> = {
            Rc::new(RefCell::new(Xorshift128Plus::new([42, 69])))
        });
        Default(DEFAULT.with(|source| source.clone()))
    }

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
