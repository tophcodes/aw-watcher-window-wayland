use mio::Token;

// Setup some tokens to allow us to identify which event is for which socket.
pub const STATE_CHANGE: Token = Token(0);
pub const TIMER: Token = Token(1);
