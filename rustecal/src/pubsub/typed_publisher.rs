use crate::pubsub::publisher::Publisher;
use crate::pubsub::types::DataTypeInfo;

pub trait PublisherMessage {
    fn datatype() -> DataTypeInfo;
    fn to_bytes(&self) -> Vec<u8>;
}

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

impl PublisherMessage for String {
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "utf-8".to_string(),
            type_name: "string".to_string(),
            descriptor: vec![],
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

pub struct TypedPublisher<T: PublisherMessage> {
    publisher: Publisher,
    _marker: std::marker::PhantomData<T>,
}

impl<T: PublisherMessage> TypedPublisher<T> {
    pub fn new(topic: &str) -> Result<Self, String> {
        let datatype = T::datatype();
        let publisher = Publisher::new(topic, datatype)?;

        Ok(Self {
            publisher,
            _marker: std::marker::PhantomData,
        })
    }

    pub fn send(&self, msg: &T) {
        self.publisher.send(&msg.to_bytes());
    }
}
