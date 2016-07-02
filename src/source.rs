use sequence::Sequence;
use value::Value;

/// A source of randomness.
pub trait Source {
    /// Read a random `u64`.
    ///
    /// The implied distribution is a discrete uniform distribution over
    /// `{0, 1, …, u64::MAX}`.
    fn read_u64(&mut self) -> u64;

    /// Read a random `f64`.
    ///
    /// The implied distribution is a continuous uniform distribution over
    /// `[0, 1]`.
    #[inline(always)]
    fn read_f64(&mut self) -> f64 {
        self.read_u64() as f64 / ::std::u64::MAX as f64
    }

    /// Read a random value.
    #[inline(always)]
    fn read<V>(&mut self) -> V where Self: Sized, V: Value {
        Value::from(self)
    }

    /// Read a sequence of random values.
    #[inline(always)]
    fn iter<'l, V>(&'l mut self) -> Sequence<'l, Self, V> where Self: Sized, V: Value {
        From::from(self)
    }
}
