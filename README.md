# Random [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The packages provides sources of randomness.

## [Documentation][doc]

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

[doc]: https://stainless-steel.github.io/random
[status-img]: https://travis-ci.org/stainless-steel/random.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/random
[version-img]: https://img.shields.io/crates/v/random.svg
[version-url]: https://crates.io/crates/random
