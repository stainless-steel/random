use Source;

/// A random value.
pub trait Value {
    /// Read a random value.
    fn read<S>(&mut S) -> Self where S: Source;
}

macro_rules! implement(
    ($reader:ident as $($kind:ty),*) => {
        $(impl Value for $kind {
            #[inline(always)]
            fn read<S>(source: &mut S) -> Self where S: Source {
                source.$reader() as $kind
            }
        })*
    }
);

implement!(read_f64 as f32, f64);
implement!(read_u64 as i8, i16, i32, i64, isize);
implement!(read_u64 as u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use {Source, Value, Xorshift128Plus};

    #[test]
    fn read() {
        let mut source = Xorshift128Plus::new([42, 69]);

        macro_rules! read(
            ($($kind:ident => [$one:expr, $two:expr],)*) => ({$(
                assert_eq!(source.read::<$kind>(), $one);
                assert_eq!($kind::read(&mut source), $two);
            )*});
        );

        read! {
            i8 => [52, -34],
            i16 => [-17348, -1036],
            i32 => [948125133, -1432682055],
            i64 => [-6330235019914458621, -4877218639256617945],
        }
    }
}
