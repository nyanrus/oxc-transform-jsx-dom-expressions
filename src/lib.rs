//! # oxc-transform-solid
//!
//! High-performance Rust-based Solid.js JSX transformer using OXC.
//!
//! This crate provides a fast alternative to the Babel-based Solid.js JSX transformer,
//! focusing solely on JSX transformation without reimplementing Solid.js library functions.

mod transformer;
mod utils;

pub use transformer::SolidTransform;

use oxc_ast::ast::Program;
use oxc_allocator::Allocator;

/// Main entry point for the Solid.js transformer
pub struct SolidJsTransformer {
    /// Configuration options for the transformer
    options: SolidTransformOptions,
}

/// Configuration options for Solid.js transformation
#[derive(Debug, Clone)]
pub struct SolidTransformOptions {
    /// Generate source maps
    pub generate_source_maps: bool,
    /// Enable development mode optimizations
    pub development: bool,
    /// Hydratable mode
    pub hydratable: bool,
    /// Module format (esm, cjs)
    pub module_format: ModuleFormat,
}

#[derive(Debug, Clone)]
pub enum ModuleFormat {
    Esm,
    Cjs,
}

impl Default for SolidTransformOptions {
    fn default() -> Self {
        Self {
            generate_source_maps: false,
            development: false,
            hydratable: false,
            module_format: ModuleFormat::Esm,
        }
    }
}

impl SolidJsTransformer {
    /// Create a new transformer with default options
    pub fn new() -> Self {
        Self {
            options: SolidTransformOptions::default(),
        }
    }

    /// Create a new transformer with custom options
    pub fn with_options(options: SolidTransformOptions) -> Self {
        Self { options }
    }

    /// Transform a program using the Solid.js transformer
    pub fn transform_program(&mut self, program: &mut Program) {
        let allocator = Allocator::default();
        let mut solid_transform = SolidTransform::new(&self.options, &allocator);
        solid_transform.transform_program(program);
    }

    /// Transform a program with custom allocator
    pub fn transform_program_with_allocator(&mut self, program: &mut Program, allocator: &Allocator) {
        let mut solid_transform = SolidTransform::new(&self.options, allocator);
        solid_transform.transform_program(program);
    }
}

impl Default for SolidJsTransformer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transformer_creation() {
        let transformer = SolidJsTransformer::new();
        assert!(!transformer.options.development);
        assert!(!transformer.options.hydratable);
    }

    #[test]
    fn test_transformer_with_options() {
        let options = SolidTransformOptions {
            development: true,
            hydratable: true,
            ..Default::default()
        };
        let transformer = SolidJsTransformer::with_options(options);
        assert!(transformer.options.development);
        assert!(transformer.options.hydratable);
    }
}
