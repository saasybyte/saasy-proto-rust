# saasy-proto-rust

Generated Rust proto types for [SaasyByte](https://github.com/saasybyte/saasybyte), an open-source real-time AI voice platform.

A types-only crate: `build.rs` compiles the canonical `.proto` files from the [saasy-proto](https://github.com/saasybyte/saasy-proto) submodule via tonic-build, re-exported as `saasy_proto_rust::{signal, sfu, shared, ...}`. It also provides bidirectional conversions between proto types and [mediasoup](https://crates.io/crates/mediasoup) types in `src/shared/conversions/`.

Consumed as a git dependency by saasy-signal, saasy-sfu, and saasy-orchestrator.

## Build

Requirements: stable Rust toolchain, `protoc` (protobuf compiler).

```bash
git submodule update --init   # saasy-proto
make build
make clippy-strict
```

Proto schemas are never defined or modified here; update them in saasy-proto, then pull the submodule forward.

## License

Apache-2.0, see [LICENSE](LICENSE).
