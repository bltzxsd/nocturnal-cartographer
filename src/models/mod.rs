use ggez::GameError;

pub mod logger;

pub type Result<T> = ::std::result::Result<T, GameError>;
