use uuid::Uuid;
use crate::types::common::Entity;
use crate::types::placement::PlacementId;

pub type FlowId = Uuid;

enum Anchor {
    None, Top, Left, Bottom, Right
}

struct Port {
    placement: PlacementId,
    anchor: Anchor
    // TODO: InterfaceElement?
}

pub struct FlowArrow {
    id: FlowId,
    from: Port,
    to: Port
}

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