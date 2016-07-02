use source::Source;

/// A random value.
pub trait Value {
    /// Create a random value from a source.
    fn from<S>(&mut S) -> Self where S: Source;
}

impl Value for f64 {
    #[inline(always)]
    fn from<S>(source: &mut S) -> Self where S: Source {
        source.read_f64()
    }
}

impl Value for u64 {
    #[inline(always)]
    fn from<S>(source: &mut S) -> Self where S: Source {
        source.read_u64()
    }
}
