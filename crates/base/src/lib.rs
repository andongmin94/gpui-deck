//! Headless form, navigation, and overlay primitives for GPUI.
//!
//! Applications own presentation. Call `init` once and host each window's
//! content in `Root` for keyboard navigation and modal focus containment.
//!
//! Derived from GPUI Kit v0.9.0 under Apache-2.0; the public surface and
//! initialization are scoped to the documented controls and their support APIs.

#[cfg(feature = "components")]
#[path = "components.rs"]
mod components;
#[cfg(feature = "components")]
pub use components::*;

#[cfg(feature = "recipe-sources")]
pub mod recipe_sources;
