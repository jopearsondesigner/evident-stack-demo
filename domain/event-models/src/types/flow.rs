use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::common::Entity;
use crate::types::placement::PlacementId;

pub type FlowId = Uuid;

#[derive(Debug, Serialize, Deserialize)]
enum Anchor {
    None, Top, Left, Bottom, Right
}

#[derive(Debug, Serialize, Deserialize)]
struct Port {
    placement: PlacementId,
    anchor: Anchor
    // TODO: InterfaceElement?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowArrow {
    id: FlowId,
    from: Port,
    to: Port
}

// TODO: enforce business rules!
impl FlowArrow {
    fn new(from: Port, to: Port) -> FlowArrow {
        FlowArrow {
            id: Uuid::new_v5(
                &from.placement,
                to.placement.as_bytes()
            ),
            from,
            to
        }
    }
}

impl Entity for FlowArrow {
    fn id(&self) -> &FlowId {
        &self.id
    }
}