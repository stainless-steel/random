use std::marker::PhantomData;

use source::Source;
use value::Value;

/// A random sequence.
pub struct Sequence<'l, S: ?Sized, V> where S: Source + 'l, V: Value + 'l {
    source: &'l mut S,
    phantom: PhantomData<&'l V>,
}

impl<'l, S, V> From<&'l mut S> for Sequence<'l, S, V> where S: Source, V: Value {
    #[inline(always)]
    fn from(source: &'l mut S) -> Self {
        Sequence { source: source, phantom: PhantomData }
    }
}

impl<'l, S, V> Iterator for Sequence<'l, S, V> where S: Source, V: Value {
    type Item = V;

    #[inline(always)]
    fn next(&mut self) -> Option<V> {
        Some(self.source.read())
    }
}
