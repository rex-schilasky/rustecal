use crate::pubsub::subscriber::Subscriber;
use crate::pubsub::types::DataTypeInfo;
use rustecal_sys::{eCAL_SDataTypeInformation, eCAL_SReceiveCallbackData, eCAL_STopicId};
use rustecal_sys::eCAL_Subscriber_SetReceiveCallback;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::slice;
use std::str;

pub trait SubscriberMessage: Sized {
    fn datatype() -> DataTypeInfo;
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

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

pub struct TypedSubscriber<T: SubscriberMessage> {
    subscriber: Subscriber,
    _phantom: PhantomData<T>,
}

impl<T: SubscriberMessage> TypedSubscriber<T> {
    pub fn new(topic: &str) -> Result<Self, String> {
        let datatype = T::datatype();

        let subscriber = Subscriber::new(topic, datatype, trampoline, std::ptr::null_mut())?;

        Ok(Self {
            subscriber,
            _phantom: PhantomData,
        })
    }

    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        CALLBACK.with(|cb| {
            *cb.borrow_mut() = Some(Box::new(move |bytes: &[u8]| {
                if let Some(decoded) = T::from_bytes(bytes) {
                    callback(decoded);
                }
            }));
        });

        unsafe {
            eCAL_Subscriber_SetReceiveCallback(
                self.subscriber.raw_handle(),
                Some(trampoline),
                std::ptr::null_mut(),
            );
        }
    }
}

use std::cell::RefCell;
use std::thread_local;

type CallbackFn = Box<dyn Fn(&[u8]) + Send + Sync + 'static>;

thread_local! {
    static CALLBACK: RefCell<Option<CallbackFn>> = RefCell::new(None);
}

extern "C" fn trampoline(
    _topic_id: *const eCAL_STopicId,
    _data_type_info: *const eCAL_SDataTypeInformation,
    data: *const eCAL_SReceiveCallbackData,
    _user_data: *mut c_void,
) {
    unsafe {
        if data.is_null() {
            return;
        }
        let msg_slice = slice::from_raw_parts((*data).buffer as *const u8, (*data).buffer_size);
        CALLBACK.with(|cb| {
            if let Some(callback) = &*cb.borrow() {
                callback(msg_slice);
            }
        });
    }
}
