//! # oxc-transform-jsx-dom-expressions
//!
//! High-performance Rust-based dom-expressions JSX transformer using OXC.
//!
//! This crate provides a fast alternative to the Babel-based dom-expressions JSX transformer,
//! focusing solely on JSX transformation without reimplementing dom-expressions library functions.

mod transformer;
mod utils;

pub use transformer::{DomExpressionsTransform, DomExpressionsTransformOptions, ModuleFormat};
