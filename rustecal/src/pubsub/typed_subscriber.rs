use crate::pubsub::subscriber::Subscriber;
use crate::pubsub::types::DataTypeInfo;
use prost::Message;
use rustecal_sys::{eCAL_SDataTypeInformation, eCAL_SReceiveCallbackData, eCAL_STopicId};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::slice;
use std::str;

/// Marker trait to enable blanket impl for prost types
pub trait IsProtobufType {}

/// A message that can be subscribed to from eCAL
pub trait SubscriberMessage: Sized {
    fn datatype() -> DataTypeInfo;
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

// === Raw bytes ===
impl SubscriberMessage for Vec<u8> {
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "raw".into(),
            type_name: "bytes".into(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Some(bytes.to_vec())
    }
}

// === UTF-8 Strings ===
impl SubscriberMessage for String {
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "utf-8".to_string(),
            type_name: "string".to_string(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        str::from_utf8(bytes).ok().map(|s| s.to_string())
    }
}

// === Protobuf Support ===
impl<T> SubscriberMessage for T
where
    T: Message + Default + IsProtobufType,
{
    fn datatype() -> DataTypeInfo {
        DataTypeInfo {
            encoding: "proto".to_string(),
            type_name: std::any::type_name::<T>().to_string(),
            descriptor: vec![],
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        T::decode(bytes).ok()
    }
}

// === Helper struct to type erase and re-box callback
struct CallbackWrapper<T: SubscriberMessage> {
    callback: Box<dyn Fn(T) + Send + Sync>,
}

impl<T: SubscriberMessage> CallbackWrapper<T> {
    fn new<F>(f: F) -> Self
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(f),
        }
    }

    fn call(&self, bytes: &[u8]) {
        if let Some(value) = T::from_bytes(bytes) {
            (self.callback)(value);
        }
    }
}

// === Typed Subscriber ===
pub struct TypedSubscriber<T: SubscriberMessage> {
    subscriber: Subscriber,
    user_data: *mut CallbackWrapper<T>,
    _phantom: PhantomData<T>,
}

impl<T: SubscriberMessage> TypedSubscriber<T> {
    pub fn new(topic: &str) -> Result<Self, String> {
        let datatype = T::datatype();

        let boxed: Box<CallbackWrapper<T>> = Box::new(CallbackWrapper::new(|_| {}));
        let user_data = Box::into_raw(boxed);

        let subscriber = Subscriber::new(topic, datatype, trampoline::<T>, user_data as *mut _)?;

        Ok(Self {
            subscriber,
            user_data,
            _phantom: PhantomData,
        })
    }

    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        unsafe {
            let _ = Box::from_raw(self.user_data); // Drop old
        }

        let boxed = Box::new(CallbackWrapper::new(callback));
        self.user_data = Box::into_raw(boxed);

        unsafe {
            rustecal_sys::eCAL_Subscriber_SetReceiveCallback(
                self.subscriber.raw_handle(),
                Some(trampoline::<T>),
                self.user_data as *mut _,
            );
        }
    }
}

impl<T: SubscriberMessage> Drop for TypedSubscriber<T> {
    fn drop(&mut self) {
        unsafe {
            rustecal_sys::eCAL_Subscriber_RemoveReceiveCallback(self.subscriber.raw_handle());
            let _ = Box::from_raw(self.user_data);
        }
    }
}

// === Trampoline ===
extern "C" fn trampoline<T: SubscriberMessage>(
    _topic_id: *const eCAL_STopicId,
    _data_type_info: *const eCAL_SDataTypeInformation,
    data: *const eCAL_SReceiveCallbackData,
    user_data: *mut c_void,
) {
    unsafe {
        if data.is_null() || user_data.is_null() {
            return;
        }

        let msg_slice = slice::from_raw_parts((*data).buffer as *const u8, (*data).buffer_size);
        let cb_wrapper = &*(user_data as *const CallbackWrapper<T>);
        cb_wrapper.call(msg_slice);
    }
}
