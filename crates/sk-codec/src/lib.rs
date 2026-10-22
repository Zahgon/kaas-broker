//! sk-codec — Kafka wire primitives, CRC32C, KIP-482 tagged fields.
//!
//! Port of the Go `internal/protocol/codec` package. Bit-perfect equivalent
//! of the Go codec: every primitive encodes and decodes byte-identically.
//!
//! # Byte-opacity contract
//!
//! RecordBatch payloads are byte-opaque throughout this crate. There is no
//! `Record` struct — batch bytes are never parsed into records. Only two
//! functions look at batch bytes at all, and neither decodes a record:
//! [`crc::verify_batch_crc`] treats the batch as opaque CRC32C input, and
//! [`recordbatch_count::count_records_in_batches`] reads only the 61-byte
//! v2 batch header to pick up `numRecords`.

pub mod crc;
pub mod errors;
pub mod primitives;
pub mod recordbatch_count;
pub mod tagged;

pub use bytes::{Bytes, BytesMut};
pub use errors::CodecError;
