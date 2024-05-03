# us-state-info-rs
United States Individual State Information
[![Docs](https://docs.rs/us-state-info-rs/badge.svg)]
[![Apache-2 licensed](https://img.shields.io/crates/l/us-state-info-rs.svg)](./LICENSE)
[![CI](https://github.com/calvinbrown085/us-state-info-rs/workflows/Rust/badge.svg)](https://github.com/calvinbrown085/us-state-info-rs/actions?query=workflow%3ARust)

A Rust implementation of the state names & abbreviations for the USA

## Usage

### Basic Usage

```rust
use us_state_info_rs::State;

fn main() {
    let iowa_state = State::Iowa;
    println!("{}-{}", iowa_state, iowa_state.abbreviation());
}
```

### Serde Serialization and Deserialization

With Serde support enabled, you can serialize and deserialize states. By default, states are serialized to their full names. If you enable the `serde_abbreviation` feature, they will be serialized to their abbreviations instead.

```rust
use serde_json::json;
use us_state_info_rs::State;

fn main() {
    let state = State::California;
    let serialized = serde_json::to_string(&state).unwrap();
    println!("Serialized: {}", serialized); // "California" or "CA" with `serde_abbreviation`

    let deserialized: State = serde_json::from_str(&serialized).unwrap(); // In this crate the deserialization can either be the 2 letter or Full name of the state.
    println!("Deserialized: {}", deserialized);
}
```