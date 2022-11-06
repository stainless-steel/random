# Random [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The packages provides sources of randomness.

## Example

```rust
use random::Source;

let mut source = random::default().seed([42, 69]);
println!("Scalar: {:?}", source.read::<f64>());
println!("Vector: {:?}", source.iter().take(2).collect::<Vec<f64>>());
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[build-img]: https://github.com/stainless-steel/random/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/random/actions/workflows/build.yml
[documentation-img]: https://docs.rs/random/badge.svg
[documentation-url]: https://docs.rs/random
[package-img]: https://img.shields.io/crates/v/random.svg
[package-url]: https://crates.io/crates/random
