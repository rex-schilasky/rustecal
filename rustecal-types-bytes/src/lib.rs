use rustecal::{PublisherMessage, SubscriberMessage};
use rustecal::pubsub::types::DataTypeInfo;

pub struct BytesMessage(pub Vec<u8>);

impl SubscriberMessage for BytesMessage {
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "raw".into(),
            type_name: "bytes".into(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(BytesMessage(bytes.to_vec()))
    }
}

impl PublisherMessage for BytesMessage {
    fn datatype() -> DataTypeInfo {
        <BytesMessage as SubscriberMessage>::datatype()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.clone()
    }
}
