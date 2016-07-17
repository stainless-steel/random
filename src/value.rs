use source::Source;

/// A random value.
pub trait Value {
    /// Create a random value from a source.
    fn from<S>(&mut S) -> Self where S: Source;
}

macro_rules! implement(
    ($reader:ident as $($kind:ty),*) => {
        $(impl Value for $kind {
            #[inline(always)]
            fn from<S>(source: &mut S) -> Self where S: Source {
                source.read_f64() as $kind
            }
        })*
    }
);

implement!(read_f64 as f32, f64);
implement!(read_u64 as i8, i16, i32, i64, isize);
implement!(read_u64 as u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use {Source, Xorshift128Plus};

    #[test]
    fn from() {
        let mut source = Xorshift128Plus::new([42, 69]);
        let _ = source.read::<i8>();
    }
}
