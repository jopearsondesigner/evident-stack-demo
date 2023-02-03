use event_models::types::{
    Audience, Command, CommandId, Described, Entity, Event, EventId, FlowArrow, FlowId, Interface,
    InterfaceId, Named, Placement, PlacementId, ReadModel, ReadModelId, Schema, SchemaId, Stream,
};
use event_models::{EventModel, EventModelId};
use std::collections::HashMap;
use uuid::Uuid;

struct ConvergentEventModel {
    id: EventModelId,
}

impl EventModel for ConvergentEventModel {
    fn new(id: EventModelId, name: String) -> Self {
        todo!()
    }

    fn interfaces(&self) -> &HashMap<InterfaceId, Interface> {
        todo!()
    }

    fn commands(&self) -> &HashMap<CommandId, Command> {
        todo!()
    }

    fn events(&self) -> &HashMap<EventId, Event> {
        todo!()
    }

    fn read_models(&self) -> &HashMap<ReadModelId, ReadModel> {
        todo!()
    }

    fn audiences(&self) -> &Vec<Audience> {
        todo!()
    }

    fn streams(&self) -> &Vec<Stream> {
        todo!()
    }

    fn placements(&self) -> &HashMap<PlacementId, Placement> {
        todo!()
    }

    fn flows(&self) -> &HashMap<FlowId, FlowArrow> {
        todo!()
    }

    fn schemas(&self) -> &HashMap<SchemaId, Schema> {
        todo!()
    }
}

impl Described for ConvergentEventModel {
    fn description(&self) -> Option<&str> {
        todo!()
    }

    fn set_description(&mut self, description: &str) {
        todo!()
    }
}

impl Named for ConvergentEventModel {
    fn name(&self) -> &str {
        todo!()
    }

    fn rename(&mut self, name: &str) {
        todo!()
    }
}

impl Entity for ConvergentEventModel {
    fn id(&self) -> &Uuid {
        todo!()
    }
}
