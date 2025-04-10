use prost::Message;
use rustecal::pubsub::{PublisherMessage, SubscriberMessage};
use rustecal::pubsub::types::DataTypeInfo;

/// Marker trait to opt-in for Protobuf support
pub trait IsProtobufType {}

/// Wrapper around a `prost::Message` to enable use with TypedPublisher and TypedSubscriber
#[derive(Debug, Clone)]
pub struct ProtobufMessage<T>(pub T);

impl<T> SubscriberMessage for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "proto".to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            descriptor: vec![], // descriptor injection planned
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        T::decode(bytes).ok().map(ProtobufMessage)
    }
}

impl<T> PublisherMessage for ProtobufMessage<T>
where
    T: Message + Default + IsProtobufType,
{
    fn datatype() -> DataTypeInfo {
        <ProtobufMessage<T> as SubscriberMessage>::datatype()
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.0.encoded_len());
        self.0
            .encode(&mut buf)
            .expect("Failed to encode protobuf message");
        buf
    }
}
