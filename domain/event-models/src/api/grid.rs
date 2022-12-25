use std::collections::HashMap;
use crate::types::audience::Audience;
use crate::types::event_model::EventModel;
use crate::types::placement::{EventPlacement, InterfacePlacement, PlacementIndex, TimelinePlacement};
use crate::types::stream::Stream;

impl EventModel {
    fn grid(&self) -> Vec<GridLane> {
        let lane_count = 1 // default audience
            + self.audiences.len() // audiences
            + 1 // timeline
            + self.streams.len() // streams
            + 1; // default stream
        let mut lanes: Vec<GridLane> = Vec::with_capacity(lane_count);
        todo!();
        lanes
    }
}

pub enum GridLane<'a> {
    DefaultAudience(HashMap<PlacementIndex, &'a InterfacePlacement>),
    Audience(&'a Audience, HashMap<PlacementIndex, &'a InterfacePlacement>),
    Timeline(HashMap<PlacementIndex, &'a TimelinePlacement>),
    Stream(&'a Stream, HashMap<PlacementIndex, &'a EventPlacement>),
    Default(&'a HashMap<PlacementIndex, &'a EventPlacement>)
}