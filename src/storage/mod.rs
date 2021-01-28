//! This is the heart of ImmuxDB, all the critical storage engine codes are in this mod.
//! The design of this storage engine is inspired by [Sled](https://github.com/spacejam/sled) and [Bitcask](https://github.com/basho/bitcask), a log-structured file format based storage engine.

/// Chain height is used to trace the order of instructions in our log file.
pub mod chain_height;

/// Call for documentation.
pub mod ecc;

/// General storage engine errors.
pub mod errors;

/// A wrapper of the storage engine, executor provide more rich types, etc.
pub mod executor;

/// The minimum unit of our log file.
pub mod instruction;

/// Iterator for instruction buffer.
pub mod instruction_iterator;

/// Core of the storage engine, all the read write logic codes are in this mod.
pub mod kv;

/// Raw key of the Key-Value storage structure.
pub mod kvkey;

/// Raw value of the Key-Value storage structure.
pub mod kvvalue;

/// File pointer points to a valid instruction.
pub mod log_pointer;

/// Reader for the log structure file.
pub mod log_reader;

/// An identifier of the log file.
pub mod log_version;

/// Writer for the log structure file.
pub mod log_writer;

/// Configuration of the storage engine.
pub mod preferences;

/// ImmuxDB supports snapshot isolation!
pub mod transaction_manager;
