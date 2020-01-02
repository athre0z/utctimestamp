utctimestamp
============

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

[crates-badge]: https://img.shields.io/crates/v/utctimestamp.svg
[crates-url]: https://crates.io/crates/utctimestamp
[docs-badge]: https://docs.rs/utctimestamp/badge.svg
[docs-url]: https://docs.rs/utctimestamp/

Simple & fast UTC time types

```toml
[dependencies]
utctimestamp = "0.1"
```

While [chrono](https://crates.io/crates/chrono) is great for dealing with time
in most cases, its 96-bit integer design can be costly when processing and storing 
large amounts of timestamp data.

This lib solves this problem by providing very simple UTC timestamps that can be
converted from and into their corresponding chrono counterpart using Rust's
`From` and `Into` traits. chrono is then used for all things that aren't expected
to occur in big batches, such as formatting and displaying the timestamps. 

#### Optional features

`serde-support` — Enable (de)serialization support with serde

