//! # rustecal
//!
//! Safe and idiomatic bindings for the [Eclipse eCAL](https://github.com/eclipse-ecal/ecal) middleware.
//!
//! ## Features
//! - High-performance publish/subscribe communication
//! - Strongly typed messaging (`StringMessage`, `BytesMessage`, `ProtobufMessage<T>`)
//! - Safe wrappers around the eCAL C API
//! - Extensible through message type crates
//!
//! ## Crate Layout
//! - [`Ecal`] – Initialization and shutdown
//! - [`TypedPublisher`] – High-level publishing
//! - [`TypedSubscriber`] – High-level subscribing
//!
//! ## Examples
//! ```no_run
//! use rustecal::{Ecal, EcalComponents};
//! use rustecal_types_string::StringMessage;
//! use rustecal::TypedPublisher;
//!
//! fn main() {
//!     Ecal::initialize(Some("example pub"), EcalComponents::DEFAULT).unwrap();
//!     let pub_ = TypedPublisher::<StringMessage>::new("hello").unwrap();
//!     pub_.send(&StringMessage("Hello from Rust".into()));
//!     Ecal::finalize();
//! }
//! ```

pub mod pubsub;
pub mod ecal;

pub use ecal::core::Ecal;
pub use ecal::components::EcalComponents;

pub use pubsub::{
    TypedSubscriber,
    SubscriberMessage,
    TypedPublisher,
    PublisherMessage,
};

// Optional if needed by demos:
pub use pubsub::publisher::Publisher;
pub use pubsub::subscriber::Subscriber;
// Service module
pub mod service;
