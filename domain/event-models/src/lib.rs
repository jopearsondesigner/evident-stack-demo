extern crate core;
#[cfg(feature = "cbor")]
extern crate serde_cbor;
extern crate url;
extern crate uuid;
#[cfg(test)]
#[macro_use]
extern crate assert_matches;

use crate::types::audience::Audience;
use crate::types::command::{Command, CommandId};
use crate::types::event::{Event, EventId};
use crate::types::flow::{FlowArrow, FlowId};
use crate::types::interface::{Interface, InterfaceId};
use crate::types::placement::PlacementPosition;
use crate::types::placement::{Placement, PlacementId};
use crate::types::read_model::{ReadModel, ReadModelId};
use crate::types::schema::HasSchema;
use crate::types::stream::Stream;
use crate::types::{Component, ComponentId, Described, Lane, LaneId, LaneIndex};
use std::collections::HashMap;
use std::fmt::Debug;
use types::flow::flow_id;
use types::schema::HasModifiableSchema;
use types::{ModifiablyDescribed, Renamable};
use uuid::Uuid;

pub mod api;
//pub mod application;
pub mod grid;
pub mod implementation;
pub mod types;

pub type EventModelId = Uuid;

pub trait EventModelCreator<T: EventModel>: Debug + Default {
    fn create(&self, id: EventModelId, name: String) -> T;
}

pub trait EventModel: Described + HasSchema + Debug {
    fn interfaces(&self) -> &HashMap<InterfaceId, Interface>;
    fn commands(&self) -> &HashMap<CommandId, Command>;
    fn events(&self) -> &HashMap<EventId, Event>;
    fn read_models(&self) -> &HashMap<ReadModelId, ReadModel>;
    fn audiences(&self) -> &Vec<Audience>;
    fn streams(&self) -> &Vec<Stream>;
    fn placements(&self) -> &HashMap<PlacementId, Placement>;
    fn flows(&self) -> &HashMap<FlowId, FlowArrow>;
}

pub trait ModifiableEventModel:
    EventModel + Renamable + ModifiablyDescribed + HasModifiableSchema
{
    // ***** Components *****

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

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn added_to_component_schema(&mut self, component_id: &ComponentId, index: u32, addition: &str);

    // Validation of presence of component_id must be performed
    //  by `decide` prior to this step
    fn deleted_from_component_schema(&mut self, component_id: &ComponentId, index: u32);

    // ***** Placements *****

    fn component_placed(&mut self, placement: &Placement);
    fn placement_moved(&mut self, position: &PlacementPosition);
    fn placement_removed(&mut self, placement_id: &PlacementId);

    // Validation of presence of placement_id must be performed
    //  by `decide` prior to this step
    fn added_to_placement_schema(&mut self, placement_id: &PlacementId, index: u32, addition: &str);

    // Validation of presence of placement_id must be performed
    //  by `decide` prior to this step
    fn deleted_from_placement_schema(&mut self, placement_id: &PlacementId, index: u32);

    // ***** Lanes *****

    fn lane_added(&mut self, lane: Lane, index: LaneIndex);
    fn lane_renamed(&mut self, lane_id: LaneId, name: &str);
    fn lane_reordered(&mut self, lane_id: LaneId, index: LaneIndex);
    fn lane_removed(&mut self, lane_id: LaneId);

    // ***** Flows *****
    fn plus_flow(&mut self, flow_arrow: FlowArrow);
    fn minus_flow_by_placement_ids(&mut self, from: &PlacementId, to: &PlacementId) {
        self.minus_flow(&flow_id(from, to));
    }
    fn minus_flow(&mut self, flow_id: &FlowId);
}
