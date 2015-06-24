extern crate probability;

pub use probability::generator::{Generator, Quantity, XorshiftPlus};

#[cfg(test)]
mod tests {
    use {Generator, XorshiftPlus};

    #[test]
    fn read() {
        let mut generator = XorshiftPlus::new([42, 42]);
        let _ = generator.next::<f64>();
        let _ = generator.next::<u64>();
    }
}
