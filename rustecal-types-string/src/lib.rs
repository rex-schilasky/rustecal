use rustecal::{PublisherMessage, SubscriberMessage};
use rustecal::pubsub::types::DataTypeInfo;
use std::str;

pub struct StringMessage(pub String);

impl SubscriberMessage for StringMessage {
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "utf-8".to_string(),
            type_name: "string".to_string(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        str::from_utf8(bytes).ok().map(|s| StringMessage(s.to_string()))
    }
}

impl PublisherMessage for StringMessage {
    fn datatype() -> DataTypeInfo {
        <StringMessage as SubscriberMessage>::datatype()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_vec()
    }
}
