#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, deny(warnings))]

//! # golem-wasi-http
//!
//! An HTTP client library that uses WASI HTTP (WebAssembly System Interface) as its backend.
//!
//! The library provides a convenient API for making HTTP requests from WASM components.
//! It supports both async and synchronous operations, with optional support for JSON and multipart forms.
//!
//! Originally started as a fork of the [reqwest](https://crates.io/crates/reqwest) library but diverged significantly
//! and dropped all the non WASI-HTTP backends.
//!
//! ## Features
//!
//! - Async and blocking Clients
//! - Plain bodies, JSON, form data, and multipart support
//! - Custom request/response body handling
//!
//! ## Optional Features
//!
//! The following are a list of [Cargo features][cargo-features] that can be
//! enabled or disabled:
//!
//! - **async**: Async client API using the [wstd](https://crates.io/crates/wstd) crate
//! - **json**: Provides serialization and deserialization for JSON bodies.
//! - **multipart**: Provides functionality for multipart forms.
//!

pub use http::header;
pub use http::Method;
pub use http::{StatusCode, Version};
pub use url::Url;

// universal mods
#[macro_use]
mod error;
mod into_url;
mod response;

pub use self::error::{Error, Result};
pub use self::into_url::IntoUrl;
pub use self::response::ResponseBuilderExt;

mod util;
mod wasi;

pub use self::wasi::{Body, Client, ClientBuilder, Request, RequestBuilder, Response};
pub use ::wasi::http::types::IncomingBody;
pub use ::wasi::io::streams::{InputStream, StreamError};

#[cfg(feature = "multipart")]
pub use self::wasi::multipart;

#[cfg(feature = "async")]
pub use crate::wasi::{CustomRequestBodyWriter, CustomRequestExecution};
