//! rustecal-pubsub: TypedPublisher & TypedSubscriber for eCAL

// Re-export core init & types
pub use rustecal_core::{Ecal, EcalComponents};

// Sub‑modules
pub mod types;
pub mod publisher;
pub mod subscriber;
pub mod typed_publisher;
pub mod typed_subscriber;

// Public API
pub use publisher::Publisher;
pub use subscriber::Subscriber;
pub use typed_publisher::TypedPublisher;
pub use typed_publisher::PublisherMessage;
pub use typed_subscriber::TypedSubscriber;
pub use typed_subscriber::SubscriberMessage;
