mod grid;

use std::collections::HashSet;
use crate::domain::types::event_model::{EventModel, EventModelId};

// TODO: implement in terms of DTOs, as appropriate

pub trait EventModelRepository {
    type Error;

    fn all_ids(&self) -> HashSet<EventModelId>;
    fn by_id(&self, id: &EventModelId) -> Option<&dyn EventModel>;
    fn by_name(&self, id: &str) -> Option<&dyn EventModel>;

    fn put_by_id(
        &self,
        id: &EventModelId,
        event_model: &dyn EventModel
    ) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
