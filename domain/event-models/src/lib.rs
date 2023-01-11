extern crate url;
extern crate uuid;
extern crate core;
#[cfg(feature = "cbor")]
extern crate serde_cbor;

use uuid::Uuid;
use std::collections::HashMap;
use types::placement::PlacementPosition;
use crate::types::{Component, ComponentId, ComponentMut, Described, Lane, LaneId, LaneIndex};
use crate::types::audience::Audience;
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::placement::{Placement, PlacementId};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::schema::{Schema, SchemaId};
use crate::types::stream::Stream;

pub mod domain;
pub mod application;
pub mod types;
pub mod grid;
pub mod default;

pub type EventModelId = Uuid;

pub trait EventModel: Described {
    fn new(id: EventModelId, name: String) -> Self;
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
    fn renamed(self, name: &str) -> Self;
    fn added_to_description(self, index: u32, addition: &str) -> Self;
    fn deleted_from_description(self, index: u32) -> Self;
}

pub trait EventModelComponentModifier: EventModelModifier {
    fn component_mut_by_id(&mut self, id: &ComponentId) -> Option<ComponentMut>;

    fn component_defined(self, component: Component) -> Self;

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn component_renamed(self, component_id: &ComponentId, name: &str) -> Self;

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn component_removed(self, component_id: &ComponentId) -> Self;

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn added_to_component_description(
        self, component_id: &ComponentId, index: u32, addition: &str
    ) -> Self;
    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn deleted_from_component_description(
        self, component_id: &ComponentId, index: u32
    ) -> Self;
}

pub trait EventModelPlacementModifier: EventModelModifier {
    fn component_placed(self, placement: &Placement) -> Self;
    fn placement_moved(
        self,
        position: &PlacementPosition
    ) -> Self;

    fn placement_removed(self, placement_id: &PlacementId) -> Self;
}

pub trait EventModelLaneModifier: EventModelModifier {
    fn lane_added(self, lane: Lane, index: LaneIndex) -> Self;
    fn lane_renamed(self, lane_id: LaneId, name: &str) -> Self;
    fn lane_reordered(self, lane_id: LaneId, index: LaneIndex) -> Self;
    fn lane_removed(self, lane_id: LaneId) -> Self;
}

pub trait EventModelFlowModifier: EventModelModifier {
    fn plus_flow(self, flow_arrow: &FlowArrow) -> Self;
    fn minus_flow_by_placement_ids(self, from: &PlacementId, to: &PlacementId) -> Self;

    fn minus_flow(self, flow_id: &FlowId) -> Self;
}

pub trait EventModelSchemaModifier: EventModelComponentModifier {
    fn schema_defined(self, schema: &Schema) -> Self;

    fn added_to_schema_definition(self, schema_id: &SchemaId, index: u32, addition: &str) -> Self;
    fn deleted_from_schema_definition(self, schema_id: &SchemaId, index: u32) -> Self;

    fn added_to_schema_description(self, schema_id: &SchemaId, index: u32, addition: &str
    ) -> Self;
    fn deleted_from_schema_description(self, schema_id: &SchemaId, index: u32) -> Self;

    fn remove_schema(self, schema_id: &SchemaId) -> Self;
}
