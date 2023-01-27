use std::collections::HashMap;

// TODO: use DTOs instead of domain types
use crate::types::audience::Audience;
use crate::types::placement::{
    EventPlacement, InterfacePlacement, PlacementIndex, TimelinePlacement,
};
use crate::types::stream::Stream;
use crate::EventModel;

fn grid(event_model: &impl EventModel) -> Grid {
    todo!()
}

pub struct DefaultAudienceLane(HashMap<PlacementIndex, InterfacePlacement>);
pub struct AudienceLane(Audience, HashMap<PlacementIndex, InterfacePlacement>);
pub struct TimelineLane(HashMap<PlacementIndex, TimelinePlacement>);
pub struct StreamLane(Stream, HashMap<PlacementIndex, EventPlacement>);
pub struct DefaultStreamLane(HashMap<PlacementIndex, EventPlacement>);

pub struct Grid {
    default_audience: DefaultAudienceLane,
    audiences: Vec<AudienceLane>,
    timeline: TimelineLane,
    streams: Vec<StreamLane>,
    default_stream: DefaultStreamLane,
}

impl Grid {
    fn lane_count(&self) -> usize {
        1 + self.audiences.len() + 1 + self.streams.len() + 1
    }

    fn column_count(&self) -> usize {
        // find max placement index from all lanes
        todo!()
    }
}
