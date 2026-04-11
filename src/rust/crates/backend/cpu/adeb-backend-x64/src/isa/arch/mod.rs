// ============================================================
// ISA Architecture Layer — x86-64 encoding/decoding
// ============================================================
// Pure architecture definitions: no language-specific logic.
// Encoder, decoder, VEX prefixes, label resolution.
// ============================================================

#[path = "../encoder.rs"]
pub mod encoder;

#[path = "../decoder.rs"]
pub mod decoder;

#[path = "../vex_emitter.rs"]
pub mod vex_emitter;

#[path = "../bit_resolver.rs"]
pub mod bit_resolver;
