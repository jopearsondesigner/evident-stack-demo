extern crate core;
#[cfg(feature = "cbor")]
extern crate serde_cbor;
extern crate url;
extern crate uuid;

use crate::types::audience::Audience;
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::placement::PlacementPosition;
use crate::types::placement::{Placement, PlacementId};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::schema::{Schema, SchemaId};
use crate::types::stream::Stream;
use crate::types::{Component, ComponentId, ComponentMut, Described, Lane, LaneId, LaneIndex};
use std::collections::HashMap;
use uuid::Uuid;

pub mod application;
pub mod default;
pub mod domain;
pub mod grid;
pub mod types;

pub type EventModelId = Uuid;

pub trait EventModel: Described {
    fn interfaces(&self) -> &HashMap<InterfaceId, Interface>;
    fn commands(&self) -> &HashMap<CommandId, Command>;
    fn events(&self) -> &HashMap<EventId, Event>;
    fn read_models(&self) -> &HashMap<ReadModelId, ReadModel>;
    fn audiences(&self) -> &Vec<Audience>;
    fn streams(&self) -> &Vec<Stream>;
    fn placements(&self) -> &HashMap<PlacementId, Placement>;
    fn flows(&self) -> &HashMap<FlowId, FlowArrow>;
    fn schemas(&self) -> &HashMap<SchemaId, Schema>;
}

pub trait EventModelModifier: EventModel {
    // Name validation must be performed by `decide` prior to this step
    fn added_to_description(&mut self, index: u32, addition: &str);
    fn deleted_from_description(&mut self, index: u32);
}

pub trait EventModelComponentModifier: EventModelModifier {
    fn component_defined(&mut self, component: Component);

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn component_renamed(&mut self, component_id: &ComponentId, name: &str);

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn component_removed(&mut self, component_id: &ComponentId);

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn added_to_component_description(
        &mut self,
        component_id: &ComponentId,
        index: u32,
        addition: &str,
    );

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn deleted_from_component_description(&mut self, component_id: &ComponentId, index: u32);
}

pub trait EventModelPlacementModifier: EventModelModifier {
    fn component_placed(&mut self, placement: &Placement);
    fn placement_moved(&mut self, position: &PlacementPosition);
    fn placement_removed(&mut self, placement_id: &PlacementId);
}

pub trait EventModelLaneModifier: EventModelModifier {
    fn lane_added(&mut self, lane: Lane, index: LaneIndex);
    fn lane_renamed(&mut self, lane_id: LaneId, name: &str);
    fn lane_reordered(&mut self, lane_id: LaneId, index: LaneIndex);
    fn lane_removed(&mut self, lane_id: LaneId);
}

pub trait EventModelFlowModifier: EventModelModifier {
    fn plus_flow(&mut self, flow_arrow: &FlowArrow);
    fn minus_flow_by_placement_ids(&mut self, from: &PlacementId, to: &PlacementId);

    fn minus_flow(&mut self, flow_id: &FlowId);
}

pub trait EventModelSchemaModifier: EventModelComponentModifier {
    fn schema_defined(&mut self, schema: &Schema);

    fn added_to_schema_definition(&mut self, schema_id: &SchemaId, index: u32, addition: &str);
    fn deleted_from_schema_definition(&mut self, schema_id: &SchemaId, index: u32);

    fn added_to_schema_description(&mut self, schema_id: &SchemaId, index: u32, addition: &str);
    fn deleted_from_schema_description(&mut self, schema_id: &SchemaId, index: u32);

    fn remove_schema(&mut self, schema_id: &SchemaId);
}
