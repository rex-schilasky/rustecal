use crate::pubsub::subscriber::Subscriber;
use crate::pubsub::types::DataTypeInfo;
use rustecal_sys::{eCAL_SDataTypeInformation, eCAL_SReceiveCallbackData, eCAL_STopicId};
use std::ffi::{c_void, CStr};
use std::marker::PhantomData;
use std::slice;

/// Trait that must be implemented for any type used with [`TypedSubscriber`].
///
/// Provides metadata and deserialization logic for a specific message type.
pub trait SubscriberMessage: Sized {
    /// Returns the metadata that describes this message type (encoding, name, optional descriptor).
    fn datatype() -> DataTypeInfo;

    /// Constructs an instance of the message type from a byte slice.
    fn from_bytes(bytes: &[u8]) -> Option<Self>;
}

/// Represents a received message with associated metadata.
///
/// This includes the deserialized message and eCAL metadata such as timestamp and topic information.
pub struct Received<T> {
    /// The decoded message of type `T`.
    pub msg: T,

    /// The name of the topic this message was received on.
    pub topic_name: String,

    /// The declared encoding format (e.g. "proto", "string", "raw").
    pub encoding: String,

    /// The declared type name of the message (may match `std::any::type_name::<T>()`).
    pub type_name: String,

    /// The send timestamp provided by the publisher (microseconds since epoch).
    pub timestamp: i64,

    /// The logical clock value at which the message was sent.
    pub clock: i64,
}

/// Internal trampoline wrapper that stores a type-erased callback for dispatching typed messages.
struct CallbackWrapper<T: SubscriberMessage> {
    callback: Box<dyn Fn(Received<T>) + Send + Sync>,
}

impl<T: SubscriberMessage> CallbackWrapper<T> {
    fn new<F>(f: F) -> Self
    where
        F: Fn(Received<T>) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(f),
        }
    }

    fn call(&self, received: Received<T>) {
        (self.callback)(received);
    }
}

/// A high-level, type-safe subscriber for a specific message type `T`.
///
/// Wraps the lower-level [`Subscriber`] to provide automatic deserialization and typed callbacks.
pub struct TypedSubscriber<T: SubscriberMessage> {
    subscriber: Subscriber,
    user_data: *mut CallbackWrapper<T>,
    _phantom: PhantomData<T>,
}

impl<T: SubscriberMessage> TypedSubscriber<T> {
    /// Creates a new typed subscriber for the specified topic.
    ///
    /// # Arguments
    ///
    /// * `topic_name` - The name of the topic to subscribe to.
    ///
    /// # Returns
    ///
    /// `Ok(Self)` if the subscriber was created successfully, or `Err` with a description.
    pub fn new(topic_name: &str) -> Result<Self, String> {
        let datatype = T::datatype();

        // Set dummy callback for construction, real callback will be assigned later
        let boxed: Box<CallbackWrapper<T>> = Box::new(CallbackWrapper::new(|_| {}));
        let user_data = Box::into_raw(boxed);

        // FIXED: remove `user_data` argument here
        let subscriber = Subscriber::new(topic_name, datatype, trampoline::<T>)?;

        Ok(Self {
            subscriber,
            user_data,
            _phantom: PhantomData,
        })
    }


    /// Registers a user callback that receives a deserialized message with metadata.
    ///
    /// This replaces any previously set callback and transfers ownership of the closure.
    ///
    /// # Arguments
    ///
    /// * `callback` - A closure accepting a [`Received<T>`] message.
    pub fn set_callback<F>(&mut self, callback: F)
    where
        F: Fn(Received<T>) + Send + Sync + 'static,
    {
        unsafe {
            // Drop the old callback
            let _ = Box::from_raw(self.user_data);
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
    /// Cleans up and removes the callback, releasing any boxed closures.
    fn drop(&mut self) {
        unsafe {
            rustecal_sys::eCAL_Subscriber_RemoveReceiveCallback(self.subscriber.raw_handle());
            let _ = Box::from_raw(self.user_data);
        }
    }
}

/// Internal trampoline for dispatching incoming messages to the registered user closure.
///
/// Converts C FFI types into Rust-safe [`Received<T>`] values and passes them to the callback.
extern "C" fn trampoline<T: SubscriberMessage>(
    topic_id: *const eCAL_STopicId,
    data_type_info: *const eCAL_SDataTypeInformation,
    data: *const eCAL_SReceiveCallbackData,
    user_data: *mut c_void,
) {
    unsafe {
        if data.is_null() || user_data.is_null() {
            return;
        }

        let msg_slice = slice::from_raw_parts((*data).buffer as *const u8, (*data).buffer_size);

        if let Some(decoded) = T::from_bytes(msg_slice) {
            let cb_wrapper = &*(user_data as *const CallbackWrapper<T>);

            let topic_name = CStr::from_ptr((*topic_id).topic_name).to_string_lossy().into_owned();
            let encoding = CStr::from_ptr((*data_type_info).encoding).to_string_lossy().into_owned();
            let type_name = CStr::from_ptr((*data_type_info).name).to_string_lossy().into_owned();

            let metadata = Received {
                msg: decoded,
                topic_name,
                encoding,
                type_name,
                timestamp: (*data).send_timestamp,
                clock: (*data).send_clock,
            };

            cb_wrapper.call(metadata);
        }
    }
}
