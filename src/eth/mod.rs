//! Legacy Ethereum-like ABI generator

#![warn(missing_docs)]

mod util;
mod log;
mod stream;
mod sink;
mod common;
#[cfg(test)]
mod tests;

pub use self::log::AsLog;
pub use self::stream::Stream;
pub use self::sink::Sink;

pub struct Bytes(pub ::lib::Vec<u8>);

/// Error for decoding rust types from stream
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	/// Invalid bool for provided input
	InvalidBool,
	/// Invalid u32 for provided input
	InvalidU8,
	/// Invalid u32 for provided input
	InvalidU16,
	/// Invalid u32 for provided input
	InvalidU32,
	/// Invalid u64 for provided input
	InvalidU64,
	/// Unexpected end of the stream
	UnexpectedEof,
	/// Invalid padding for fixed type
	InvalidPadding,
	/// Other error
	Other,
}

/// Abi type trait
pub trait AbiType : Sized {
	/// Insantiate type from data stream
	fn decode(stream: &mut Stream) -> Result<Self, Error>;

	/// Push type to data sink
	fn encode(self, sink: &mut Sink);

	/// Whether type has fixed length or not
	const IS_FIXED: bool;
}

/// Endpoint interface for contracts
pub trait EndpointInterface {
	/// Dispatch payload for regular method
	fn dispatch(&mut self, payload: &[u8]) -> ::lib::Vec<u8>;

	/// Dispatch constructor payload
	fn dispatch_ctor(&mut self, payload: &[u8]);
}
