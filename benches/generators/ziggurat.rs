use test::{Bencher, black_box};

use random::Generator;
use random::generators::Ziggurat;
use random::sources::XorshiftPlus;

#[bench]
fn gaussian(bencher: &mut Bencher) {
    let mut generator = Ziggurat::new(XorshiftPlus::new([42, 42]));
    bencher.iter(|| {
        black_box(generator.gaussian());
    });
}
