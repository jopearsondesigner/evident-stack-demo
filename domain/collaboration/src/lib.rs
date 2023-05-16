use uuid::Uuid;

pub mod client;
pub mod server;

pub type DocumentId = Uuid;
pub type UserId = String;
pub type PeerId = String;
pub type Base64String = String;
pub type EventId = Uuid;
