use event_models::implementation::in_memory::InMemoryEventModel;
use event_models::types::schema::{HasModifiableSchema, HasSchema};
use event_models::types::{
    Audience, Command, CommandId, Described, Entity, Event, EventId, FlowArrow, FlowId, Interface,
    InterfaceId, ModifiablyDescribed, Named, Placement, PlacementId, PlacementPosition, ReadModel,
    ReadModelId, Renamable, Schema, Stream,
};
use event_models::{
    EventModel, EventModelData, EventModelId, EventModelState, ModifiableEventModel,
};
use std::collections::HashMap;
use uuid::Uuid;

use converge::{random_node, Interpreter, Node, OpId, OpSet, Patch};

#[derive(Clone, Debug)]
pub enum Op {
    Named(String),
}

#[derive(Debug, Clone)]
pub struct ConvergentEventModel {
    node: Node,
    opset: OpSet<Op>,
    value: InMemoryEventModel,
}

impl ConvergentEventModel {
    pub fn new(id: EventModelId, name: String, node: Node, mut opset: OpSet<Op>) -> Self {
        let mut patch: Patch<Op> = Patch::default();
        patch.insert(opset.next_id(&node), Op::Named(name.to_string()));
        opset.apply_patch(patch);
        let value = ConvergentEventModel::interpret(
            InMemoryEventModel::new(id.to_owned(), "".to_string()),
            &opset,
        );
        ConvergentEventModel { node, value, opset }
    }

    pub fn apply_patch(mut self, patch: Patch<Op>) -> Self {
        self.opset.apply_patch(patch);
        self.value = Self::interpret(self.value, &self.opset);
        self
    }
}

impl Default for ConvergentEventModel {
    fn default() -> Self {
        Self::new(
            Uuid::new_v4(),
            "New Event Model".into(),
            random_node(),
            OpSet::default(),
        )
    }
}

// Converge Implementation

impl Interpreter<Op> for ConvergentEventModel {
    type Interpretation = InMemoryEventModel;

    fn evolve(mut state: Self::Interpretation, _id: &OpId, op: &Op) -> Self::Interpretation {
        match op {
            Op::Named(name) => {
                state.rename(name);
                state
            }
        }
    }
}

// Event Model Implementation

impl Entity for ConvergentEventModel {
    fn id(&self) -> &Uuid {
        self.value.id()
    }
}

impl Named for ConvergentEventModel {
    fn name(&self) -> &str {
        self.value.name()
    }
}

impl Renamable for ConvergentEventModel {
    fn rename(&mut self, name: &str) {
        todo!()
    }
}

impl Described for ConvergentEventModel {
    fn description(&self) -> &str {
        self.value.description()
    }
}

impl EventModelData for ConvergentEventModel {
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
}

#[derive(Clone, Debug)]
pub struct ConvergentCreationDetails {
    node: Node,
}

impl ConvergentCreationDetails {
    pub fn new(node: Node) -> Self {
        ConvergentCreationDetails { node }
    }
}

impl EventModel for ConvergentEventModel {
    type CreationDetails = ConvergentCreationDetails;

    fn create(initial: EventModelState<Self>, id: EventModelId, name: String) -> Self {
        match initial {
            EventModelState::BeforeCreation(details) => {
                Self::new(id, name, details.node, OpSet::default())
            }
            _ => panic!("Illegal initial state when creating Event Model"),
        }
    }
}

impl ModifiableEventModel for ConvergentEventModel {
    fn component_defined(&mut self, component: event_models::types::Component) {
        todo!()
    }

    fn component_renamed(&mut self, component_id: &event_models::types::ComponentId, name: &str) {
        todo!()
    }

    fn component_removed(&mut self, component_id: &event_models::types::ComponentId) {
        todo!()
    }

    fn added_to_component_description(
        &mut self,
        component_id: &event_models::types::ComponentId,
        index: usize,
        addition: &str,
    ) {
        todo!()
    }

    fn deleted_from_component_description(
        &mut self,
        component_id: &event_models::types::ComponentId,
        index: usize,
        count: usize,
    ) {
        todo!()
    }

    fn added_to_component_schema(
        &mut self,
        component_id: &event_models::types::ComponentId,
        index: usize,
        addition: &str,
    ) {
        todo!()
    }

    fn deleted_from_component_schema(
        &mut self,
        component_id: &event_models::types::ComponentId,
        index: usize,
        count: usize,
    ) {
        todo!()
    }

    fn component_placed(&mut self, placement: &Placement) {
        todo!()
    }

    fn placement_moved(&mut self, position: &PlacementPosition) {
        todo!()
    }

    fn placement_removed(&mut self, placement_id: &PlacementId) {
        todo!()
    }

    fn added_to_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        addition: &str,
    ) {
        todo!()
    }

    fn deleted_from_placement_schema(
        &mut self,
        placement_id: &PlacementId,
        index: usize,
        count: usize,
    ) {
        todo!()
    }

    fn lane_added(
        &mut self,
        lane: event_models::types::Lane,
        index: event_models::types::LaneIndex,
    ) {
        todo!()
    }

    fn lane_renamed(&mut self, lane_id: event_models::types::LaneId, name: &str) {
        todo!()
    }

    fn lane_reordered(
        &mut self,
        lane_id: event_models::types::LaneId,
        index: event_models::types::LaneIndex,
    ) {
        todo!()
    }

    fn lane_removed(&mut self, lane_id: event_models::types::LaneId) {
        todo!()
    }

    fn plus_flow(&mut self, flow_arrow: FlowArrow) {
        todo!()
    }

    fn minus_flow(&mut self, flow_id: &FlowId) {
        todo!()
    }

    // fn renamed(self, name: &str) -> Self {
    //     let mut patch: Patch<Op> = HashMap::new();
    //     patch.insert(self.opset.next_id(&self.node), Op::Named(name.to_owned()));
    //     self.apply_patch(patch)
    // }

    // fn added_to_description(self, index: u32, addition: &str) -> Self {
    //     todo!()
    // }

    // fn deleted_from_description(self, index: u32) -> Self {
    //     todo!()
    // }
}

impl ModifiablyDescribed for ConvergentEventModel {
    fn description_mut(&mut self) -> &mut String {
        todo!()
    }
}

impl HasSchema for ConvergentEventModel {
    fn schema(&self) -> &Schema {
        todo!()
    }
}

impl HasModifiableSchema for ConvergentEventModel {
    fn schema_mut(&mut self) -> &mut Schema {
        todo!()
    }

    fn set_schema(&mut self, schema: Schema) {
        todo!()
    }
}

// impl EventModelComponentModifier for ConvergentEventModel {
//     fn component_mut_by_id(
//         &mut self,
//         id: &event_models::types::ComponentId,
//     ) -> Option<event_models::types::ComponentMut> {
//         todo!()
//     }

//     fn component_defined(self, component: event_models::types::Component) -> Self {
//         todo!()
//     }

//     fn component_renamed(
//         self,
//         component_id: &event_models::types::ComponentId,
//         name: &str,
//     ) -> Self {
//         todo!()
//     }

//     fn component_removed(self, component_id: &event_models::types::ComponentId) -> Self {
//         todo!()
//     }

//     fn added_to_component_description(
//         self,
//         component_id: &event_models::types::ComponentId,
//         index: u32,
//         addition: &str,
//     ) -> Self {
//         todo!()
//     }

//     fn deleted_from_component_description(
//         self,
//         component_id: &event_models::types::ComponentId,
//         index: u32,
//     ) -> Self {
//         todo!()
//     }
// }

// impl EventModelPlacementModifier for ConvergentEventModel {
//     fn component_placed(self, placement: &Placement) -> Self {
//         todo!()
//     }

//     fn placement_moved(self, position: &PlacementPosition) -> Self {
//         todo!()
//     }

//     fn placement_removed(self, placement_id: &PlacementId) -> Self {
//         todo!()
//     }
// }

// impl EventModelLaneModifier for ConvergentEventModel {
//     fn lane_added(
//         self,
//         lane: event_models::types::Lane,
//         index: event_models::types::LaneIndex,
//     ) -> Self {
//         todo!()
//     }

//     fn lane_renamed(self, lane_id: event_models::types::LaneId, name: &str) -> Self {
//         todo!()
//     }

//     fn lane_reordered(
//         self,
//         lane_id: event_models::types::LaneId,
//         index: event_models::types::LaneIndex,
//     ) -> Self {
//         todo!()
//     }

//     fn lane_removed(self, lane_id: event_models::types::LaneId) -> Self {
//         todo!()
//     }
// }

// impl EventModelFlowModifier for ConvergentEventModel {
//     fn plus_flow(self, flow_arrow: &FlowArrow) -> Self {
//         todo!()
//     }

//     fn minus_flow_by_placement_ids(self, from: &PlacementId, to: &PlacementId) -> Self {
//         todo!()
//     }

//     fn minus_flow(self, flow_id: &FlowId) -> Self {
//         todo!()
//     }
// }

// impl EventModelSchemaModifier for ConvergentEventModel {
//     fn schema_defined(self, schema: &Schema) -> Self {
//         todo!()
//     }

//     fn added_to_schema_definition(self, schema_id: &SchemaId, index: u32, addition: &str) -> Self {
//         todo!()
//     }

//     fn deleted_from_schema_definition(self, schema_id: &SchemaId, index: u32) -> Self {
//         todo!()
//     }

//     fn added_to_schema_description(self, schema_id: &SchemaId, index: u32, addition: &str) -> Self {
//         todo!()
//     }

//     fn deleted_from_schema_description(self, schema_id: &SchemaId, index: u32) -> Self {
//         todo!()
//     }

//     fn remove_schema(self, schema_id: &SchemaId) -> Self {
//         todo!()
//     }
// }
