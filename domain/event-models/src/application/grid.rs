use std::collections::HashMap;

// TODO: use DTOs instead of domain types
use crate::domain::types::audience::Audience;
use crate::domain::types::event_model::EventModel;
use crate::domain::types::placement::{EventPlacement, InterfacePlacement, PlacementIndex, TimelinePlacement};
use crate::domain::types::stream::Stream;

fn grid(event_model: &dyn EventModel) -> Vec<GridLane> {
    let lane_count = 1 // default audience
        + event_model.audiences().len() // audiences
        + 1 // timeline
        + event_model.streams().len() // streams
        + 1; // default stream
    let mut lanes: Vec<GridLane> = Vec::with_capacity(lane_count);
    todo!();
    lanes
}

// TODO: implement in terms of owned DTO versions of these types,
//  rather than references to domain types
pub enum GridLane<'a> {
    DefaultAudience(HashMap<PlacementIndex, &'a InterfacePlacement>),
    Audience(&'a Audience, HashMap<PlacementIndex, &'a InterfacePlacement>),
    Timeline(HashMap<PlacementIndex, &'a TimelinePlacement>),
    Stream(&'a Stream, HashMap<PlacementIndex, &'a EventPlacement>),
    Default(&'a HashMap<PlacementIndex, &'a EventPlacement>)
}
