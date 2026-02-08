# Multivibe Core

Scope: Hardware-agnostic logic and shared data structures.

This crate contains the pure logic of Multivibe. It defines how audio samples
are processed, the structure of the messaging protocol, and the core traits that
hardware-specific crates must implement.

- Audio Pipeline: Defines PCM/Opus handling and buffer management.
- Networking Protocol: Shared logic for UDP Multicast and WebSocket payload structures.
- Traits: Defines the AudioSource and Broadcaster interfaces.
