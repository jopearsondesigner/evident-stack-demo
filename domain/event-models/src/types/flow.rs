use std::error::Error;
use std::fmt::{Display, Formatter};
use uuid::Uuid;
use crate::types::Entity;
use crate::EventModel;
use crate::types::placement::PlacementId;

pub type FlowId = Uuid;

#[derive(Debug, Default, Clone, PartialEq)]
enum Anchor {
    #[default]
    None,
    Top, Left, Bottom, Right
}

#[derive(Debug, Clone, PartialEq)]
struct Port {
    placement: PlacementId,
    anchor: Anchor
    // TODO: InterfaceElement?
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowArrow {
    id: FlowId,
    from: Port,
    to: Port
}

#[derive(Debug, Clone, PartialEq)]
pub struct IllegalFlowArrow(String);

impl Display for IllegalFlowArrow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for IllegalFlowArrow {}

// TODO: enforce business rules!
impl FlowArrow {
    fn new(model: &dyn EventModel, from: Port, to: Port) -> Result<FlowArrow, Box<dyn Error>> {
        let from_placement = model.placements().get(&from.placement);
        let to_placement = model.placements().get(&to.placement);
        todo!();
        Ok(FlowArrow {
            id: Uuid::new_v5(
                &from.placement,
                to.placement.as_bytes()
            ),
            from,
            to
        })
    }

    // fun build(
    //             model: EventModel,
    //             from: FlowPort,
    //             to: FlowPort
    //         ): FlowArrow {
    //             val fromPlacement = model.placements[from.placementId]
    //                 ?: throw IllegalArgumentException("From placement doesn't exist")
    //             val toPlacement = model.placements[to.placementId]
    //                 ?: throw IllegalArgumentException("To placement doesn't exist")
    //             if (toPlacement.index < fromPlacement.index)
    //                 throw IllegalArgumentException("Flows cannot go backward")
    //             if (model.getFlow(from.placementId, to.placementId) != null)
    //                 throw IllegalArgumentException("Flow already connects these two placements")
    //             return when (fromPlacement) {
    //                 is InterfacePlacement -> when (toPlacement) {
    //                     is CommandPlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Interfaces can only flow to Commands")
    //                 }
    //
    //                 is CommandPlacement -> when (toPlacement) {
    //                     is EventPlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Commands can only flow to Events")
    //                 }
    //
    //                 is EventPlacement -> when (toPlacement) {
    //                     is CommandPlacement, is ReadModelPlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Events can only flow to Commands or Read Models")
    //                 }
    //
    //                 is ReadModelPlacement -> when (toPlacement) {
    //                     is InterfacePlacement -> FlowArrow(from, to)
    //                     else -> throw IllegalArgumentException("Read Models can only flow to Interfaces")
    //                 }
    //             }
    //         }
}

impl Entity for FlowArrow {
    fn id(&self) -> &FlowId {
        &self.id
    }
}
