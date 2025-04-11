# Introduction

Welcome to **rustecal**, a safe and idiomatic Rust binding for [Eclipse eCAL](https://github.com/eclipse-ecal/ecal).

This project is designed to bring fast and reliable pub-sub and service-based IPC to Rust-based applications in domains such as robotics, automotive, embedded, and distributed systems.

rustecal wraps the low-level `ecal_c` C API and builds ergonomic, type-safe abstractions for working with:

- Publishers and Subscribers
- Strongly typed message serialization (e.g., Strings, Bytes, Protobuf)
- Cross-platform interprocess communication

This documentation will guide you through usage, architecture, and message type support.