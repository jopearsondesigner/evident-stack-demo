use std::collections::HashMap;
use std::ops::Deref;
use uuid::Uuid;
use crate::types::audience::Audience;
use crate::types::command::{Command, CommandId};
use crate::types::common::{Described, Entity, Named};
use crate::types::event::{Event, EventId};
use crate::types::event_model::{EventModel, EventModelId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::placement::{Placement, PlacementId};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::schema::Schema;
use crate::types::stream::Stream;
use crate::types::text::Text;

struct DefaultEventModel {
    id: EventModelId,
    name: String,
    description: Box<dyn Text>,
    interfaces: HashMap<InterfaceId, Interface>,
    commands: HashMap<CommandId, Command>,
    events: HashMap<EventId, Event>,
    read_models: HashMap<ReadModelId, ReadModel>,
    audiences: Vec<Audience>,
    streams: Vec<Stream>,
    placements: HashMap<PlacementId, Placement>,
    flows: HashMap<FlowId, FlowArrow>,
    shared_schema: Schema,
}

impl Entity for DefaultEventModel {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

impl Named for DefaultEventModel {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Described for DefaultEventModel {
    fn description(&self) -> &str {
        let desc = &self.description;
        desc.deref().into()
    }
}

impl EventModel for DefaultEventModel {
    fn interfaces(&self) -> HashMap<&InterfaceId, &Interface> {
        let mut ret = HashMap::with_capacity(self.interfaces.len());
        for (k, v) in self.interfaces.iter() {
            ret.insert(k, v);
        }
        ret
    }

    fn commands(&self) -> HashMap<&CommandId, &Command> {
        let mut ret = HashMap::with_capacity(self.commands.len());
        for (k, v) in self.commands.iter() {
            ret.insert(k, v);
        }
        ret
    }

    fn events(&self) -> HashMap<&EventId, &Event> {
        let mut ret = HashMap::with_capacity(self.events.len());
        for (k, v) in self.events.iter() {
            ret.insert(k, v);
        }
        ret
    }

    fn read_models(&self) -> HashMap<&ReadModelId, &ReadModel> {
        let mut ret = HashMap::with_capacity(self.read_models.len());
        for (k, v) in self.read_models.iter() {
            ret.insert(k, v);
        }
        ret
    }

    fn audiences(&self) -> Vec<&Audience> {
        todo!()
    }

    fn streams(&self) -> Vec<&Stream> {
        todo!()
    }

    fn placements(&self) -> HashMap<&PlacementId, &Placement> {
        todo!()
    }

    fn flows(&self) -> HashMap<&FlowId, &FlowArrow> {
        todo!()
    }

    fn shared_schema(&self) -> &Schema {
        &self.shared_schema
    }
}