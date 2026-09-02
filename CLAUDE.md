# saasy-proto-rust

## Commands
- `make build` — build crate (triggers proto generation via `build.rs`)
- `make check` — fast compilation check (no codegen)
- `make clippy` / `make clippy-strict` — lint / lint with `-D warnings`
- `make fmt` — format code
- `make clean` — clean build artifacts

## Conventions
- **Module pattern**: each proto module has a `v1` submodule using `tonic::include_proto!("saasy.<module>.v1")`, then re-exports types at the module level. Follow this pattern for new modules.
- **Conversion pattern**: bidirectional `From`/`TryFrom` impls between proto types and `mediasoup` types, organized in `src/shared/conversions/`. Proto → mediasoup uses `TryFrom` (can fail), mediasoup → proto uses `From` (infallible) or `TryFrom` where narrowing is needed.
- **Error type**: single `ConversionError` enum with `thiserror::Error` in `src/shared/conversions/error.rs`. All conversion failures use variants from this enum.
- **Proto source**: canonical `.proto` files live in the `saasy-proto/` git submodule (GitHub). Never define or modify proto schemas in this repo — update them upstream, then `cd saasy-proto && git pull`.
- **build.rs**: compiles all `.proto` files via `tonic-build`. When adding a new proto, add both the `compile_protos` entry and a `cargo:rerun-if-changed` line.

## Service Boundaries
- **Consumed by saasy-signal** (git dep): uses `signal`, `sfu`, `shared` modules.
- **Consumed by saasy-sfu** (git dep): uses `sfu`, `shared` modules.
- **Consumed by rust-orchestrator** (git dep): uses all modules.
- **Proto schema from saasy-proto** (git submodule): do not define or modify proto schemas here.
- **Does not own**: service logic, media forwarding, AI inference, auth, usage tracking — this crate is types only.
