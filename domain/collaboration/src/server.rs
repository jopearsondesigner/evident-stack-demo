use std::collections::HashMap;

use automerge::{
    sync::{Message, State, SyncDoc},
    AutoCommit,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use epoch::decider::{DeciderWithContext, Event, Evolver};
use uuid::Uuid;

use crate::{
    Base64String, CollaborationError, DocumentId, Email, EventId, MaybeEmailAddress, MaybeRole,
    PeerId, Role, UserId,
};

#[derive(Debug, Clone)]
pub struct Peer(PeerId, State);

#[derive(Debug, Clone)]
pub struct Grant {
    grantor_id: UserId,
    user_id: UserId,
    role: Role,
}

#[derive(Debug, Clone)]
pub struct Invitation {
    document_id: DocumentId,
    invitor_id: UserId,
    invited_email: Email,
    invited_role: Role,
}

#[derive(Debug, Clone)]
pub struct PeerDocument {
    pub document_id: DocumentId,
    pub document: AutoCommit,
    pub peer: Option<Peer>,
    pub current_roles: HashMap<UserId, Role>,
    pub roles_granted: HashMap<UserId, Grant>,
    pub roles_revoked: HashMap<UserId, UserId>,
    pub current_invitations: HashMap<Email, Invitation>,
    pub invitations_added: HashMap<Email, Invitation>,
    pub invitations_removed: HashMap<Email, UserId>,
}

#[derive(Debug, Clone)]
pub enum CollaborativeDocument {
    BeforeCreation,
    PeerDocument(PeerDocument),
    Deleted(DocumentId),
}

#[derive(Debug)]
pub enum CollaborationCommand {
    Sync {
        document_id: DocumentId,
        user_id: UserId,
        peer_id: PeerId,
        message_string: Base64String,
    },
    AppendPatch {
        document_id: DocumentId,
        user_id: UserId,
        patch: Base64String,
    },
    InviteUser {
        document_id: DocumentId,
        invitor_id: UserId,
        email_string: MaybeEmailAddress,
        role_string: MaybeRole,
    },
    AcceptInvitation {
        document_id: DocumentId,
        user_email: MaybeEmailAddress,
        user_id: UserId,
    },
    // TODO: support directly adding users, or only via invitation?
    // GrantAccess {
    //     document_id: DocumentId,
    //     adder_id: UserId,
    //     added_id: UserId,
    //     added_role: MaybeRole,
    // },
    RevokeAccess {
        document_id: DocumentId,
        revoker_id: UserId,
        user_id: UserId,
    },
    DeleteDocument {
        document_id: DocumentId,
        deleter_id: UserId,
    },
}

#[derive(Debug)]
pub enum CollaborationEvent {
    DocumentCreated {
        event_id: EventId,
        document_id: DocumentId,
        creator_id: UserId,
        peer_id: PeerId,
        initial_sync_message: Message,
    },
    SyncMessageReceived {
        event_id: EventId,
        document_id: DocumentId,
        user_id: UserId,
        peer_id: PeerId,
        sync_message: Message,
    },
    PatchAppended {
        event_id: EventId,
        document_id: DocumentId,
        user_id: UserId,
        patch: Vec<u8>,
    },
    UserInvited {
        event_id: EventId,
        document_id: DocumentId,
        invitor_id: UserId,
        user_email: Email,
        user_role: Role,
    },
    InvitationRemoved {
        event_id: EventId,
        document_id: DocumentId,
        remover_id: UserId,
        user_email: Email,
    },
    AccessGranted {
        event_id: EventId,
        document_id: DocumentId,
        grantor_id: UserId,
        user_id: UserId,
        user_role: Role,
    },
    AccessRevoked {
        event_id: EventId,
        document_id: DocumentId,
        revoker_id: UserId,
        user_id: UserId,
    },
    DocumentDeleted {
        event_id: EventId,
        document_id: DocumentId,
        deleter_id: UserId,
    },
}

impl Event for CollaborationEvent {
    type EntityId = Uuid;

    fn event_type(&self) -> String {
        match self {
            CollaborationEvent::DocumentCreated { .. } => "DocumentCreated".to_string(),
            CollaborationEvent::SyncMessageReceived { .. } => "SyncMessageReceived".to_string(),
            CollaborationEvent::PatchAppended { .. } => "PatchAppended".to_string(),
            CollaborationEvent::UserInvited { .. } => "UserInvited".to_string(),
            CollaborationEvent::InvitationRemoved { .. } => "InvitationRemoved".to_string(),
            CollaborationEvent::AccessGranted { .. } => "AccessGranted".to_string(),
            CollaborationEvent::AccessRevoked { .. } => "AccessRevoked".to_string(),
            CollaborationEvent::DocumentDeleted { .. } => "DocumentDeleted".to_string(),
        }
    }

    fn get_id(&self) -> Self::EntityId {
        match self {
            CollaborationEvent::DocumentCreated { event_id, .. } => *event_id,
            CollaborationEvent::SyncMessageReceived { event_id, .. } => *event_id,
            CollaborationEvent::PatchAppended { event_id, .. } => *event_id,
            CollaborationEvent::UserInvited { event_id, .. } => *event_id,
            CollaborationEvent::InvitationRemoved { event_id, .. } => *event_id,
            CollaborationEvent::AccessGranted { event_id, .. } => *event_id,
            CollaborationEvent::AccessRevoked { event_id, .. } => *event_id,
            CollaborationEvent::DocumentDeleted { event_id, .. } => *event_id,
        }
    }
}

#[derive(Debug)]
pub struct CollaborationDecider;

impl DeciderWithContext for CollaborationDecider {
    type Ctx = (); // TODO: lookup user role by id?

    type Cmd = CollaborationCommand;

    type Err = CollaborationError;

    fn decide(
        _ctx: &Self::Ctx,
        state: &Self::State,
        cmd: &Self::Cmd,
    ) -> Result<Vec<Self::Evt>, Self::Err> {
        match cmd {
            CollaborationCommand::Sync {
                document_id,
                user_id,
                peer_id,
                message_string,
            } => match state {
                CollaborativeDocument::Deleted(_) => Err(CollaborationError::IllegalState),
                CollaborativeDocument::BeforeCreation => {
                    let message_data = BASE64_STANDARD
                        .decode(message_string)
                        .map_err(|_| CollaborationError::MessageParseError)?;
                    let message = Message::decode(&message_data)
                        .map_err(|_| CollaborationError::MessageParseError)?;
                    Ok(vec![CollaborationEvent::DocumentCreated {
                        event_id: Uuid::new_v4(),
                        document_id: *document_id,
                        creator_id: user_id.to_owned(),
                        peer_id: peer_id.to_owned(),
                        initial_sync_message: message,
                    }])
                }
                CollaborativeDocument::PeerDocument(PeerDocument { current_roles, .. }) => {
                    if let Some(user_role) = current_roles.get(user_id) {
                        match user_role {
                            Role::Owner | Role::Admin | Role::Editor => {
                                let message_data = BASE64_STANDARD
                                    .decode(message_string)
                                    .map_err(|_| CollaborationError::MessageParseError)?;
                                let message = Message::decode(&message_data)
                                    .map_err(|_| CollaborationError::MessageParseError)?;
                                Ok(vec![CollaborationEvent::SyncMessageReceived {
                                    event_id: Uuid::new_v4(),
                                    document_id: *document_id,
                                    user_id: user_id.to_owned(),
                                    peer_id: peer_id.to_owned(),
                                    sync_message: message,
                                }])
                            }
                            Role::Viewer => Err(CollaborationError::NotAuthorized),
                        }
                    } else {
                        Err(CollaborationError::NotAuthorized)
                    }
                }
            },
            CollaborationCommand::AppendPatch {
                document_id,
                user_id,
                patch,
            } => match state {
                CollaborativeDocument::BeforeCreation => Err(CollaborationError::IllegalState),
                CollaborativeDocument::Deleted(_) => Err(CollaborationError::IllegalState),
                CollaborativeDocument::PeerDocument(PeerDocument { current_roles, .. }) => {
                    if let Some(user_role) = current_roles.get(user_id) {
                        let patch_data = BASE64_STANDARD
                            .decode(patch)
                            .map_err(|_| CollaborationError::MessageParseError)?;
                        let success = Ok(vec![CollaborationEvent::PatchAppended {
                            event_id: Uuid::new_v4(),
                            document_id: *document_id,
                            user_id: user_id.to_owned(),
                            patch: patch_data,
                        }]);
                        match user_role {
                            Role::Owner => success,
                            Role::Admin => match user_role {
                                Role::Admin | Role::Editor | Role::Viewer => success,
                                _ => Err(CollaborationError::NotAuthorized),
                            },
                            _ => Err(CollaborationError::NotAuthorized),
                        }
                    } else {
                        Err(CollaborationError::NotAuthorized)
                    }
                }
            },
            CollaborationCommand::InviteUser {
                document_id,
                invitor_id,
                email_string,
                role_string,
            } => match state {
                CollaborativeDocument::BeforeCreation => Err(CollaborationError::IllegalState),
                CollaborativeDocument::Deleted(_) => Err(CollaborationError::IllegalState),
                CollaborativeDocument::PeerDocument(PeerDocument { current_roles, .. }) => {
                    if let Some(invitor_role) = current_roles.get(invitor_id) {
                        let invited_email = Email::new(email_string)?;
                        let invited_role: Role = role_string.try_into()?;
                        let success = Ok(vec![CollaborationEvent::UserInvited {
                            event_id: Uuid::new_v4(),
                            document_id: *document_id,
                            invitor_id: invitor_id.to_owned(),
                            user_email: invited_email,
                            user_role: invited_role.to_owned(),
                        }]);
                        match invitor_role {
                            Role::Owner => success,
                            Role::Admin => match invited_role {
                                Role::Admin | Role::Editor | Role::Viewer => success,
                                _ => Err(CollaborationError::NotAuthorized),
                            },
                            _ => Err(CollaborationError::NotAuthorized),
                        }
                    } else {
                        Err(CollaborationError::NotAuthorized)
                    }
                }
            },
            CollaborationCommand::AcceptInvitation {
                user_id,
                user_email,
                ..
            } => match state {
                CollaborativeDocument::BeforeCreation => Err(CollaborationError::IllegalState),
                CollaborativeDocument::Deleted(_) => Err(CollaborationError::IllegalState),
                CollaborativeDocument::PeerDocument(PeerDocument {
                    document_id,
                    current_invitations,
                    ..
                }) => {
                    let email = Email::new(user_email)?;
                    if let Some(invitation) = current_invitations.get(&email) {
                        Ok(vec![
                            CollaborationEvent::AccessGranted {
                                event_id: Uuid::new_v4(),
                                document_id: *document_id,
                                grantor_id: invitation.invitor_id.to_owned(),
                                user_id: user_id.to_owned(),
                                user_role: invitation.invited_role.to_owned(),
                            },
                            CollaborationEvent::InvitationRemoved {
                                event_id: Uuid::new_v4(),
                                document_id: *document_id,
                                remover_id: user_id.to_owned(),
                                user_email: email.to_owned(),
                            },
                        ])
                    } else {
                        Err(CollaborationError::IllegalState)
                    }
                }
            },
            CollaborationCommand::RevokeAccess {
                document_id,
                revoker_id,
                user_id,
            } => match state {
                CollaborativeDocument::BeforeCreation => Err(CollaborationError::IllegalState),
                CollaborativeDocument::Deleted(_) => Err(CollaborationError::IllegalState),
                CollaborativeDocument::PeerDocument(PeerDocument { current_roles, .. }) => {
                    let maybe_revoker_role = current_roles.get(revoker_id);
                    let maybe_user_role = current_roles.get(revoker_id);
                    match (maybe_revoker_role, maybe_user_role) {
                        (Some(revoker_role), Some(user_role)) => {
                            let success = Ok(vec![CollaborationEvent::AccessRevoked {
                                event_id: Uuid::new_v4(),
                                document_id: *document_id,
                                revoker_id: revoker_id.to_owned(),
                                user_id: user_id.to_owned(),
                            }]);
                            if let (Role::Owner, _)
                            | (Role::Admin, Role::Admin)
                            | (Role::Admin, Role::Editor)
                            | (Role::Admin, Role::Viewer) = (revoker_role, user_role)
                            {
                                success
                            } else {
                                Err(CollaborationError::NotAuthorized)
                            }
                        }
                        (Some(_), None) => Err(CollaborationError::IllegalState),
                        (None, None) | (None, Some(_)) => Err(CollaborationError::NotAuthorized),
                    }
                }
            },
            CollaborationCommand::DeleteDocument {
                document_id,
                deleter_id,
            } => match state {
                CollaborativeDocument::BeforeCreation => Err(CollaborationError::IllegalState),
                CollaborativeDocument::Deleted(_) => Err(CollaborationError::IllegalState),
                CollaborativeDocument::PeerDocument(PeerDocument { current_roles, .. }) => {
                    if let Some(Role::Owner) | Some(Role::Admin) = current_roles.get(deleter_id) {
                        Ok(vec![CollaborationEvent::DocumentDeleted {
                            event_id: Uuid::new_v4(),
                            document_id: *document_id,
                            deleter_id: deleter_id.to_owned(),
                        }])
                    } else {
                        Err(CollaborationError::NotAuthorized)
                    }
                }
            },
        }
    }
}

impl Evolver for CollaborationDecider {
    type State = CollaborativeDocument;

    type Evt = CollaborationEvent;

    fn evolve(state: Self::State, event: &Self::Evt) -> Self::State {
        match event {
            CollaborationEvent::DocumentCreated {
                document_id,
                creator_id,
                peer_id,
                initial_sync_message,
                ..
            } => {
                match state {
                    CollaborativeDocument::PeerDocument(_) => state,
                    CollaborativeDocument::Deleted(_) => state,
                    CollaborativeDocument::BeforeCreation => {
                        let mut document = AutoCommit::new();
                        let mut state = State::new();
                        document
                            .sync()
                            .receive_sync_message(&mut state, initial_sync_message.to_owned())
                            .unwrap(); // TODO: what to do about errors?
                        let user_id = creator_id.to_owned();
                        CollaborativeDocument::PeerDocument(PeerDocument {
                            document_id: *document_id,
                            document,
                            peer: Some(Peer(peer_id.to_owned(), state)),
                            current_roles: Default::default(),
                            roles_granted: HashMap::from([(
                                user_id.to_owned(),
                                Grant {
                                    grantor_id: user_id.to_owned(),
                                    user_id,
                                    role: Role::Owner,
                                },
                            )]),
                            roles_revoked: Default::default(),
                            current_invitations: Default::default(),
                            invitations_added: Default::default(),
                            invitations_removed: Default::default(),
                        })
                    }
                }
            }
            CollaborationEvent::SyncMessageReceived {
                sync_message,
                peer_id,
                ..
            } => {
                match state {
                    CollaborativeDocument::BeforeCreation => state,
                    CollaborativeDocument::Deleted(_) => state,
                    CollaborativeDocument::PeerDocument(mut collab_doc) => {
                        let Peer(peer_id, mut peer_state) = match collab_doc.peer {
                            Some(state) => state,
                            None => Peer(peer_id.to_string(), State::new()),
                        };
                        collab_doc
                            .document
                            .sync()
                            .receive_sync_message(&mut peer_state, sync_message.to_owned())
                            .unwrap(); // TODO: what to do about an error here?
                        collab_doc.peer = Some(Peer(peer_id, peer_state));
                        CollaborativeDocument::PeerDocument(collab_doc)
                    }
                }
            }
            CollaborationEvent::PatchAppended { patch, .. } => match state {
                CollaborativeDocument::BeforeCreation => state,
                CollaborativeDocument::Deleted(_) => state,
                CollaborativeDocument::PeerDocument(mut collab_doc) => {
                    collab_doc.document.load_incremental(patch).unwrap(); // TODO: what to do about an error here?
                    CollaborativeDocument::PeerDocument(collab_doc)
                }
            },
            CollaborationEvent::UserInvited {
                document_id,
                invitor_id,
                user_email,
                user_role,
                ..
            } => match state {
                CollaborativeDocument::BeforeCreation => state,
                CollaborativeDocument::Deleted(_) => state,
                CollaborativeDocument::PeerDocument(mut peer_doc) => {
                    peer_doc.invitations_added.insert(
                        user_email.to_owned(),
                        Invitation {
                            document_id: *document_id,
                            invitor_id: invitor_id.to_owned(),
                            invited_email: user_email.to_owned(),
                            invited_role: user_role.to_owned(),
                        },
                    );
                    CollaborativeDocument::PeerDocument(peer_doc)
                }
            },
            CollaborationEvent::InvitationRemoved {
                remover_id,
                user_email,
                ..
            } => match state {
                CollaborativeDocument::BeforeCreation => state,
                CollaborativeDocument::Deleted(_) => state,
                CollaborativeDocument::PeerDocument(mut peer_doc) => {
                    peer_doc
                        .invitations_removed
                        .insert(user_email.to_owned(), remover_id.to_owned());
                    CollaborativeDocument::PeerDocument(peer_doc)
                }
            },
            CollaborationEvent::AccessGranted {
                user_id,
                user_role,
                grantor_id,
                ..
            } => match state {
                CollaborativeDocument::BeforeCreation => state,
                CollaborativeDocument::Deleted(_) => state,
                CollaborativeDocument::PeerDocument(mut peer_doc) => {
                    peer_doc.roles_granted.insert(
                        user_id.to_owned(),
                        Grant {
                            grantor_id: grantor_id.to_owned(),
                            user_id: user_id.to_owned(),
                            role: user_role.to_owned(),
                        },
                    );
                    CollaborativeDocument::PeerDocument(peer_doc)
                }
            },
            CollaborationEvent::AccessRevoked {
                user_id,
                revoker_id,
                ..
            } => match state {
                CollaborativeDocument::BeforeCreation => state,
                CollaborativeDocument::Deleted(_) => state,
                CollaborativeDocument::PeerDocument(mut peer_doc) => {
                    peer_doc
                        .roles_revoked
                        .insert(user_id.to_owned(), revoker_id.to_owned());
                    CollaborativeDocument::PeerDocument(peer_doc)
                }
            },
            CollaborationEvent::DocumentDeleted { document_id, .. } => match state {
                CollaborativeDocument::BeforeCreation => state,
                CollaborativeDocument::Deleted(_) => state,
                CollaborativeDocument::PeerDocument(_) => {
                    CollaborativeDocument::Deleted(*document_id)
                }
            },
        }
    }
}
