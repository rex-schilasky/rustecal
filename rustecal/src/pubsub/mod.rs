//! Typed and untyped eCAL publish-subscribe API.
//!
//! This module provides both low-level and high-level publish-subscribe functionality.
//!
//! ## Modules
//!
//! - [`publisher`] and [`subscriber`] offer low-level wrappers over the C eCAL pub/sub API.
//! - [`typed_publisher`] and [`typed_subscriber`] offer type-safe, generic Rust APIs with
//!   automatic serialization and callback handling.
//!
//! ## Re-exports
//!
//! The traits and wrappers from the typed API are re-exported at this level for convenience:
//!
//! ```rust
//! use rustecal::TypedPublisher;
//! use rustecal::TypedSubscriber;
//! ```

pub mod types;
pub mod publisher;
pub mod subscriber;

pub mod typed_subscriber;
pub mod typed_publisher;

// Publicly re-export high-level typed pub/sub traits and wrappers
pub use typed_subscriber::{TypedSubscriber, SubscriberMessage};
pub use typed_publisher::{TypedPublisher, PublisherMessage};
