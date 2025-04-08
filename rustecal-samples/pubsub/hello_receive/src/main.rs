use rustecal::{Ecal, Subscriber};
use rustecal::pubsub::types::{
    DataTypeInfo, FfiReceiveCallbackData, FfiTopicId, FfiDataTypeInfo,
};
use std::ffi::c_void;
use std::{slice, str};

extern "C" fn receive_callback(
    _topic_id: *const FfiTopicId,
    _data_type_info: *const FfiDataTypeInfo,
    data: *const FfiReceiveCallbackData,
    _user_arg: *mut c_void,
) {
    unsafe {
        if !data.is_null() {
            let msg = slice::from_raw_parts((*data).buffer as *const u8, (*data).buffer_size);
            if let Ok(text) = str::from_utf8(msg) {
                println!("Received: {}", text);
            } else {
                println!("Received invalid UTF-8");
            }
        }
    }
}

fn main() {
    Ecal::initialize(Some("minimal string subscriber rust")).unwrap();

    let datatype = DataTypeInfo {
        encoding:   "utf-8".to_string(),
        type_name:  "string".to_string(),
        descriptor: b"".to_vec(),
    };

    let _subscriber = Subscriber::new(
        "hello",
        datatype,
        receive_callback,
        std::ptr::null_mut(),
    )
    .expect("Failed to create subscriber");

    println!("Listening for messages on 'hello'...");

    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
