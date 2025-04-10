use crate::pubsub::subscriber::Subscriber;
use crate::pubsub::types::DataTypeInfo;
use rustecal_sys::{eCAL_SDataTypeInformation, eCAL_SReceiveCallbackData, eCAL_STopicId};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::slice;

/// Trait that must be implemented for any type used with `TypedSubscriber`
pub trait SubscriberMessage: Sized {
    fn datatype() -> DataTypeInfo;
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

/// Internal callback wrapper for type erasure
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

/// A type-safe high-level eCAL subscriber
pub struct TypedSubscriber<T: SubscriberMessage> {
    subscriber: Subscriber,
    user_data: *mut CallbackWrapper<T>,
    _phantom: PhantomData<T>,
}

impl<T: SubscriberMessage> TypedSubscriber<T> {
    pub fn new(topic: &str) -> Result<Self, String> {
        let datatype = T::datatype();

        // Initially dummy callback, will be replaced in set_callback
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
            let _ = Box::from_raw(self.user_data); // Drop old one
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

/// eCAL callback trampoline to dispatch to typed closure
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
