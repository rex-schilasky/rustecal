use crate::pubsub::subscriber::Subscriber;
use crate::pubsub::types::DataTypeInfo;
use rustecal_sys::{eCAL_SDataTypeInformation, eCAL_SReceiveCallbackData, eCAL_STopicId};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::slice;
use std::str;
use prost::Message;

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
            descriptor: vec![], // Add descriptor bytes if needed
        }
    }

    fn from_bytes(bytes: &[u8]) -> Option<Self> {
        T::decode(bytes).ok()
    }
}

// === Internal wrapper to pass closure to callback ===
type CallbackFn<T> = Box<dyn Fn(T) + Send + Sync>;

struct CallbackHolder<T: SubscriberMessage> {
    callback: CallbackFn<T>,
}

// === TypedSubscriber wrapper ===
pub struct TypedSubscriber<T: SubscriberMessage> {
    subscriber: Subscriber,
    user_data: *mut c_void,
    _phantom: PhantomData<T>,
}

impl<T: SubscriberMessage> TypedSubscriber<T> {
    pub fn new(topic: &str) -> Result<Self, String> {
        let holder: Box<CallbackHolder<T>> = Box::new(CallbackHolder {
            callback: Box::new(|_msg: T| {}),
        });
        let user_data = Box::into_raw(holder) as *mut c_void;
    
        let datatype = T::datatype();
        let subscriber = Subscriber::new(topic, datatype, trampoline::<T>, user_data)?;
    
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
        // Replace the boxed callback
        let holder = Box::new(CallbackHolder {
            callback: Box::new(callback),
        });
        let new_user_data = Box::into_raw(holder) as *mut c_void;

        unsafe {
            rustecal_sys::eCAL_Subscriber_SetReceiveCallback(
                self.subscriber.raw_handle(),
                Some(trampoline::<T>),
                new_user_data,
            );

            // Free old one to avoid leak
            let _ = Box::from_raw(self.user_data as *mut CallbackHolder<T>);
            self.user_data = new_user_data;
        }
    }
}

impl<T: SubscriberMessage> Drop for TypedSubscriber<T> {
    fn drop(&mut self) {
        if !self.user_data.is_null() {
            unsafe {
                let _ = Box::from_raw(self.user_data as *mut CallbackHolder<T>);
            }
        }
    }
}

// === Trampoline: Dispatch to Boxed Fn ===
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
        if let Some(decoded) = T::from_bytes(msg_slice) {
            let holder = &*(user_data as *const CallbackHolder<T>);
            (holder.callback)(decoded);
        }
    }
}
