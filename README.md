# Nature of Code in Rust

Implementing concepts from Daniel Shiffman's _Nature of Code_ using Rust and Macroquad.

## Structure

```
crates/
├── common/           # Shared utilities
├── 00_randomness/
│   └── examples/     # All examples as binaries
├── 01_vectors/
│   └── examples/
└── 02_particles/
    └── examples/
```

Each crate contains multiple examples in its `examples/` directory.

## Running Examples

From the project root:

```bash
# List all examples in a crate
cargo run -p 00_randomness --example

# Run a specific example
cargo run -p 00_randomness --example random_walk
cargo run -p 01_vectors --example bouncing_ball
```
