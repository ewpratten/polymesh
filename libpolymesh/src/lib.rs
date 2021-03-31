//! LibPolyMesh handles everything related to the PolyMesh (`.pmf`) file format
//!
//! The most common functionality is provided through the prelude, which can be imported using:
//! 
//! ```rust
//! use libpolymesh::prelude as pmf;
//! ```

pub mod util;
pub mod common;
pub mod create;
pub mod read;
pub mod write;
pub mod prelude;