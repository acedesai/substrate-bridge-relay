#[derive(Debug, derive_more::Display)]
pub enum Error {
	#[display(fmt = "invalid RPC URL: {}", _0)]
	UrlError(String),
	#[display(fmt = "RPC response indicates invalid chain state: {}", _0)]
	InvalidChainState(String),
	#[display(fmt = "could not make RPC call: {}", _0)]
	RPCError(String),
	#[display(fmt = "could not connect to RPC URL: {}", _0)]
	WsConnectionError(String),
	#[display(fmt = "unexpected client event from RPC URL {}: {:?}", _0, _1)]
	UnexpectedClientEvent(String, String),
	#[display(fmt = "serialization error: {}", _0)]
	SerializationError(serde_json::error::Error),
}

impl std::error::Error for Error {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			Error::SerializationError(err) => Some(err),
			_ => None,
		}
	}
}
