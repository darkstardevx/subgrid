# System Architecture

The **SUBGRID** control panel engine is designed around a decoupled, low-overhead micro-frontend architecture. It prioritizes fine-grained reactivity, secure execution environments, and zero-cost abstraction primitives.

## Component Topologies

### 1. Frontend Runtime Node
- **Core Vector**: Compiled to `wasm32-unknown-unknown` target pipelines via Rust and the Leptos 0.7 framework.
- **Rendering Primitives**: Client-side reactive interface management without virtual DOM overhead, utilizing atomic signal tracks (`signal()`) for instantaneous component state propagation.
- **Aesthetic Environment**: Cyber-noir styling with a bioluminescent accent palette (`#ff007f`, `#b829c2`, `#00ffcc`) driven by Tailwind CSS utilities.

### 2. Upstream Network Channels
- **Asynchronous Data Vector**: Native WASM-bindgen HTTP request brokers (`gloo-net`) querying downstream JSON payloads from external APIs (such as GitHub profile metrics).
- **Concurrency Guardrails**: Strict thread-isolation mechanisms via pinned synchronization primitives. Thread-unsafe wrappers are explicitly handled using precise dereferencing boundaries (`*wrapper`) to ensure clean memory layout profiles inside the web runtime.
