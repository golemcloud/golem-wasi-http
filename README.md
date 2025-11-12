# golem-wasi-http

[![crates.io](https://img.shields.io/crates/v/golem-wasi-http.svg)](https://crates.io/crates/golem-wasi-http)
[![Documentation](https://docs.rs/golem-wasi-http/badge.svg)](https://docs.rs/golem-wasi-http)
[![Apache-2 licensed](https://img.shields.io/crates/l/golem-wasi-http.svg)](./LICENSE-APACHE)

Started as a fork of [reqwest](https://docs.rs/reqwest) to add a WASI-HTTP backend, now it only contains this new
backend, having a similar but not fully compatible API to the original.

To be used in [Golem](https://golem.cloud) components, or any other WASM environment that provides the WASI HTTP 0.2
host API.
