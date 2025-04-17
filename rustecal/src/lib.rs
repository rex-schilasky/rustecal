//! rustecal: all‑in‑one eCAL bindings

// —————————————————————————————————————————————————————————————————————————————
// Core initialization & types (always available)
pub use rustecal_core::{Ecal, EcalComponents};

// —————————————————————————————————————————————————————————————————————————————
// Pub/Sub API (requires the `pubsub` feature)
#[cfg(feature = "pubsub")]
pub mod pubsub {
    //! Typed and untyped Publisher/Subscriber
    pub use rustecal_pubsub::*;
}

#[cfg(feature = "pubsub")]
pub use rustecal_pubsub::{
    // low‑level handles
    Publisher, Subscriber,
    // typed wrappers
    TypedPublisher, PublisherMessage,
    TypedSubscriber, SubscriberMessage,
};

// —————————————————————————————————————————————————————————————————————————————
// Service RPC API (requires the `service` feature)
#[cfg(feature = "service")]
pub mod service {
    //! RPC server & client, plus shared types
    pub use rustecal_service::*;
}

#[cfg(feature = "service")]
pub use rustecal_service::{
    // server & client entrypoints
    ServiceServer, ServiceClient, ClientInstance,
    // request/response types
    ServiceRequest, ServiceResponse,
};

#[cfg(feature = "service")]
pub use rustecal_service::types::{
    // metadata & callback signature
    MethodInfo, ServiceCallback, CallState,
};
