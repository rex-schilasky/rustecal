# Architecture Overview

## Overview

`rustecal` builds safe wrappers around the C-based eCAL API, offering two layers:

1. **Low-level Bindings (`rustecal-sys`)**:
   - Generated via `bindgen`
   - Exposes raw `unsafe` functions

2. **Safe High-Level API (`rustecal`)**:
   - Encapsulates lifecycle management
   - Provides typed pub/sub
   - Modular message type support

## High-Level Modules

- `Ecal` – initialization/finalization of runtime
- `Publisher`/`Subscriber` – low-level pub/sub
- `TypedPublisher`/`TypedSubscriber` – type-safe wrappers
- `DataTypeInfo` – describes encoding, type name, and optional descriptor

## Message Types as Crates

Each type support lives in a separate crate to reduce dependency bloat:

- `rustecal-types-bytes`
- `rustecal-types-string`
- `rustecal-types-protobuf`