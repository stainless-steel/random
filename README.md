# Random [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The packages provides sources of randomness.

## [Documentation][docs]

## Example

```rust
use random::Source;

let mut source = random::default().seed([42, 69]);

let one = source.read::<f64>();
let two = source.iter().take(2).collect::<Vec<f64>>();
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: https://img.shields.io/crates/v/random.svg
[version-url]: https://crates.io/crates/random
[status-img]: https://travis-ci.org/stainless-steel/random.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/random
[docs]: https://stainless-steel.github.io/random
