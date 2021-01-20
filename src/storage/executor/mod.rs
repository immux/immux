//! Executor is a wrapper around the underlayer storage engine. Executor provide more rich types,
//! simple DML(index), the concept of grouping, etc.

pub mod command;
pub mod errors;
pub mod executor;
pub mod grouping_label;
pub mod outcome;
pub mod predicate;
pub mod unit_content;
pub mod unit_key;
