# Service Client Example

This example demonstrates how to call a **Mirror Service** using `rustecal`.

## Example Code

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal_service::ServiceClient;

fn main() {
    Ecal::initialize(Some("mirror_client"), EcalComponents::DEFAULT).unwrap();

    let client = ServiceClient::new("mirror_service").unwrap();

    let request_data = b"Hello, Service!";
    let timeout = std::time::Duration::from_millis(500);

    while Ecal::ok() {
        if let Some(response) = client.call(request_data, timeout) {
            println!("Received response: {:?}", String::from_utf8_lossy(&response));
        } else {
            println!("Service call timed out.");
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```
