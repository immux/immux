//! ImmuxDB Server supports both http requests and tcp connections.

/// ImmuxDB errors.
pub mod errors;

/// Message is the object which is used to be send from server threads(http or tcp) to storage
/// engine thread.
pub mod message;

/// Http server and tcp server.
pub mod server;

/// Tcp response, which might be an output or an error.
pub mod tcp_response;
