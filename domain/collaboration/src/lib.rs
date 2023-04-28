use email_address::EmailAddress;
use uuid::Uuid;

pub mod client;
pub mod server;

pub type DocumentId = Uuid;
pub type UserId = String;
pub type MaybeEmailAddress = String;
pub type MaybeRole = String;
pub type PeerId = String;
pub type Base64String = String;
pub type EventId = Uuid;

#[derive(Debug)]
pub enum CollaborationError {
    IllegalState,
    MessageParseError,
    InvalidEmailAddress(String),
    InvalidRole(String),
    NotAuthorized,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub(crate) fn new(s: &str) -> Result<Self, CollaborationError> {
        if EmailAddress::is_valid(s) {
            Ok(Email(s.to_string()))
        } else {
            Err(CollaborationError::InvalidEmailAddress(s.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
}

const OWNER: &str = "owner";
const ADMIN: &str = "admin";
const EDITOR: &str = "editor";
const VIEWER: &str = "viewer";

impl TryFrom<String> for Role {
    type Error = CollaborationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let s: &str = &value;
        s.try_into()
    }
}

impl TryFrom<&String> for Role {
    type Error = CollaborationError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let s: &str = value;
        s.try_into()
    }
}

impl TryFrom<&str> for Role {
    type Error = CollaborationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            OWNER => Ok(Role::Owner),
            ADMIN => Ok(Role::Admin),
            EDITOR => Ok(Role::Editor),
            VIEWER => Ok(Role::Viewer),
            _ => Err(CollaborationError::InvalidRole(value.to_owned())),
        }
    }
}
