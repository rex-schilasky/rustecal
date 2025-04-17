//! rustecal: all‑in‑one eCAL bindings

// Always initialize core
pub use rustecal_core::{Ecal, EcalComponents};

// Conditionally re‑export Pub/Sub
#[cfg(feature = "pubsub")]
pub use rustecal_pubsub::{
    Publisher, TypedPublisher, PublisherMessage,
    Subscriber, TypedSubscriber, SubscriberMessage,
};

// Conditionally re‑export Service
#[cfg(feature = "service")]
pub use rustecal_service::{
    ServiceServer, ServiceRequest, ServiceResponse,
    ServiceClient, ClientInstance,
};
#[cfg(feature = "service")]
pub use rustecal_service::types::{MethodInfo, ServiceCallback, CallState};
