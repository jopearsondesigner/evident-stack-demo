use event_models::implementation::in_memory::InMemoryEventModel;
use event_models::types::schema::{HasModifiableSchema, HasSchema};
use event_models::types::{
    Audience, Command, CommandId, Described, Entity, Event, EventId, FlowArrow, FlowId, Interface,
    InterfaceId, Named, Placement, PlacementId, PlacementPosition, ReadModel, ReadModelId,
    Renamable, Schema, Stream, ModifiablyDescribed,
};
use event_models::{EventModel, EventModelId, ModifiableEventModel};
use rand::random;
use std::collections::HashMap;
use uuid::Uuid;

use crate::converge::opset::InMemoryOpSet;
use crate::converge::{Id, Interpreter, Node, OpSet, Patch};

#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
enum Op {
    Named(String),
}

#[derive(Debug)]
struct ConvergentEventModel<S: OpSet<Op>> {
    node: Node,
    opset: S,
    value: InMemoryEventModel,
}

impl<S: OpSet<Op>> ConvergentEventModel<S> {
    fn new(id: EventModelId, name: &str, node: Node, mut opset: S) -> Self {
        let mut patch: Patch<Op> = HashMap::new();
        patch.insert(opset.next_id(&node), Op::Named(name.to_string()));
        opset.apply_patch(patch);
        let value = ConvergentEventModel::<S>::interpret(
            InMemoryEventModel::new(id.to_owned(), "".to_string()),
            &opset,
        );
        ConvergentEventModel { node, value, opset }
    }

    fn apply_patch(mut self, patch: Patch<Op>) -> Self {
        self.opset.apply_patch(patch);
        self.value = Self::interpret(self.value, &self.opset);
        self
    }
}

impl Default for ConvergentEventModel<InMemoryOpSet<Op>> {
    fn default() -> Self {
        Self::new(
            Uuid::new_v4(),
            "New Event Model",
            random::<u32>(),
            InMemoryOpSet::default(),
        )
    }
}

// Converge Implementation

impl<S: OpSet<Op>> Interpreter<Op> for ConvergentEventModel<S> {
    type Interpretation = InMemoryEventModel;

    fn evolve(mut state: Self::Interpretation, _id: &Id, op: &Op) -> Self::Interpretation {
        match op {
            Op::Named(name) => {
                state.rename(name);
                state
            },
        }
    }
}

// Event Model Implementation

impl<S: OpSet<Op>> Entity for ConvergentEventModel<S> {
    fn id(&self) -> &Uuid {
        self.value.id()
    }
}

impl<S: OpSet<Op>> Named for ConvergentEventModel<S> {
    fn name(&self) -> &str {
        self.value.name()
    }
}

impl<S: OpSet<Op>> Renamable for ConvergentEventModel<S> {
    fn rename(&mut self, name: &str) {
        todo!()
    }
}

impl<S: OpSet<Op>> Described for ConvergentEventModel<S> {
    fn description(&self) -> &str {
        self.value.description()
    }
}

impl<S: OpSet<Op>> EventModel for ConvergentEventModel<S> {
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

impl<S: OpSet<Op>> ModifiableEventModel for ConvergentEventModel<S> {
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
        index: u32,
        addition: &str,
    ) {
        todo!()
    }

    fn deleted_from_component_description(&mut self, component_id: &event_models::types::ComponentId, index: u32) {
        todo!()
    }

    fn added_to_component_schema(&mut self, component_id: &event_models::types::ComponentId, index: u32, addition: &str) {
        todo!()
    }

    fn deleted_from_component_schema(&mut self, component_id: &event_models::types::ComponentId, index: u32) {
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

    fn added_to_placement_schema(&mut self, placement_id: &PlacementId, index: u32, addition: &str) {
        todo!()
    }

    fn deleted_from_placement_schema(&mut self, placement_id: &PlacementId, index: u32) {
        todo!()
    }

    fn lane_added(&mut self, lane: event_models::types::Lane, index: event_models::types::LaneIndex) {
        todo!()
    }

    fn lane_renamed(&mut self, lane_id: event_models::types::LaneId, name: &str) {
        todo!()
    }

    fn lane_reordered(&mut self, lane_id: event_models::types::LaneId, index: event_models::types::LaneIndex) {
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

impl<S: OpSet<Op>> ModifiablyDescribed for ConvergentEventModel<S> {
    fn set_description(&mut self, description: &str) {
        todo!()
    }

    fn add_to_description(&mut self, index: u32, addition: &str) {
        todo!()
    }

    fn delete_from_description(&mut self, index: u32) {
        todo!()
    }
}


impl <S: OpSet<Op>> HasSchema for ConvergentEventModel<S> {
    fn schema(&self) -> &Schema {
        todo!()
    }
}

impl <S: OpSet<Op>> HasModifiableSchema for ConvergentEventModel<S> {
    fn schema_mut(&mut self) -> &mut Schema {
        todo!()
    }

    fn set_schema(&mut self, schema: Schema) {
        todo!()
    }
}

// impl<S: OpSet<Op>> EventModelComponentModifier for ConvergentEventModel<S> {
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

// impl<S: OpSet<Op>> EventModelPlacementModifier for ConvergentEventModel<S> {
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

// impl<S: OpSet<Op>> EventModelLaneModifier for ConvergentEventModel<S> {
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

// impl<S: OpSet<Op>> EventModelFlowModifier for ConvergentEventModel<S> {
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

// impl<S: OpSet<Op>> EventModelSchemaModifier for ConvergentEventModel<S> {
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
