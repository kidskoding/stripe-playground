## Stripe Playground

a Rust based "plug and play" TUI for exploring the Stripe API using your own test keys

- provides interactive flows for working with core Stripe concepts like customers, payment intents, and Checkout Sessions
- exposes the raw JSON responses for learning and debugging

Built with the official `stripe-rust` SDK and is structured into 3 components: configuration, Stripe integration, and the TUI, effectively demonstrating production-style architecture and error handling with Rust
