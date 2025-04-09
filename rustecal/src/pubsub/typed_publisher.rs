use crate::pubsub::publisher::Publisher;
use crate::pubsub::types::DataTypeInfo;
use std::marker::PhantomData;
use prost::Message;

/// Marker trait to explicitly mark protobuf types for publishing
pub trait IsProtobufType {}

/// Trait for types that can be published via eCAL
pub trait PublisherMessage {
    fn datatype() -> DataTypeInfo;
    fn to_bytes(&self) -> Vec<u8>;
}

// === Raw bytes ===
impl PublisherMessage for Vec<u8> {
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "raw".into(),
            type_name: "bytes".into(),
            descriptor: vec![],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.clone()
    }
}

// === UTF-8 Strings ===
impl PublisherMessage for String {
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "utf-8".into(),
            type_name: "string".into(),
            descriptor: vec![],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

// === Protobuf Support ===
impl<T> PublisherMessage for T
where
    T: Message + IsProtobufType,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "proto".into(),
            type_name: std::any::type_name::<T>().to_string(),
            descriptor: vec![], // Optional: populate with descriptor bytes
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(self.encoded_len());
        self.encode(&mut buf).expect("Protobuf encoding failed");
        buf
    }
}

// === TypedPublisher wrapper ===
pub struct TypedPublisher<T: PublisherMessage> {
    publisher: Publisher,
    _phantom: PhantomData<T>,
}

impl<T: PublisherMessage> TypedPublisher<T> {
    pub fn new(topic: &str) -> Result<Self, String> {
        let datatype = T::datatype();
        let publisher = Publisher::new(topic, datatype)?;

        Ok(Self {
            publisher,
            _phantom: PhantomData,
        })
    }

    pub fn send(&self, message: &T) {
        let bytes = message.to_bytes();
        self.publisher.send(&bytes);
    }
}
