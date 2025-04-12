# Ecal Lifecycle

The `Ecal` struct manages initialization and finalization of the eCAL system.

## Example

```rust
use rustecal::Ecal;

let ecal = Ecal::initialize("my_rust_app")?;
// use publishers or subscribers
drop(ecal); // finalizes on drop
```
