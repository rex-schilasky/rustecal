# Service Server Example

This example shows how to implement a basic **Mirror Service Server** using `rustecal`.

The server receives a request, logs it, and sends it back as a response.

## Example Code

```rust
use rustecal::{Ecal, EcalComponents};
use rustecal_service::{ServiceServer, ReceivedRequest};

fn main() {
    Ecal::initialize(Some("mirror_server"), EcalComponents::DEFAULT).unwrap();

    let _server = ServiceServer::new("mirror_service", move |req: ReceivedRequest| {
        let request_str = String::from_utf8_lossy(req.request());
        println!("Received request: {}", request_str);

        // Respond with the same data
        req.respond(req.request());
    }).expect("Failed to create service server");

    // Keep the server running
    while Ecal::ok() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ecal::finalize();
}
```
