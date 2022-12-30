use crate::{EventModel, EventModelModifier, EventModelComponentModifier,
            EventModelPlacementModifier, EventModelLaneModifier,
            EventModelFlowModifier, EventModelSchemaModifier};
use crate::domain::commands::EventModelCommand;
use crate::domain::events::EventModelEvent;
use crate::types::errors::EventModelError;

mod read_models;
mod events;
mod commands;

fn decide<T>(
    command: &EventModelCommand,
    state: Option<T>
) -> Result<Vec<EventModelEvent>, EventModelError>
where
    T: EventModel
{
    match command {
        EventModelCommand::Create(_) => {}
        EventModelCommand::Rename(_) => {}
        EventModelCommand::AddToDescription(_, _) => {}
        EventModelCommand::DeleteFromDescription(_) => {}
        EventModelCommand::AddAudience(_, _) => {}
        EventModelCommand::RenameAudience(_, _) => {}
        EventModelCommand::ReorderAudience(_, _) => {}
        EventModelCommand::RemoveAudience(_) => {}
        EventModelCommand::AddStream(_, _) => {}
        EventModelCommand::RenameStream(_, _) => {}
        EventModelCommand::ReorderStream(_, _) => {}
        EventModelCommand::RemoveStream(_) => {}
        EventModelCommand::DefineAndPlaceInterface(_, _, _) => {}
        EventModelCommand::DefineAndPlaceCommand(_, _) => {}
        EventModelCommand::DefineAndPlaceEvent(_, _, _) => {}
        EventModelCommand::DefineAndPlaceReadModel(_, _) => {}
        EventModelCommand::RenamePlacement(_, _) => {}
        EventModelCommand::MoveInterfacePlacement(_, _, _) => {}
        EventModelCommand::MoveTimelinePlacement(_, _) => {}
        EventModelCommand::MoveEventPlacement(_, _, _) => {}
        EventModelCommand::RemovePlacement(_) => {}
        EventModelCommand::DuplicateInterfacePlacement(_, _, _) => {}
        EventModelCommand::DuplicateTimelinePlacement(_, _) => {}
        EventModelCommand::DuplicateEventPlacement(_, _, _) => {}
        EventModelCommand::RenameComponent(_, _) => {}
        EventModelCommand::AddToComponentDescription() => {}
        EventModelCommand::DeleteFromComponentDescription() => {}
    }
    todo!()
}

// TODO: could split input state type from output state type
fn evolve<T>(
    state: Option<T>,
    event: EventModelEvent
) -> Option<T>
    where
        T: EventModel + EventModelModifier + EventModelComponentModifier
        + EventModelLaneModifier + EventModelPlacementModifier
        + EventModelFlowModifier + EventModelSchemaModifier
{
    match state {
        None => match event {
            EventModelEvent::Created(id, name) => {
                Some(EventModel::new(id, name))
            }
            _ => None
        },
        Some(mut model) => match event {
            EventModelEvent::Created(_, _) => Some(model),
            EventModelEvent::Renamed(_, name) => {
                Some(model.renamed(&name))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
