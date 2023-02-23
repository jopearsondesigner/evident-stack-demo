// use std::collections::HashSet;
// use crate::{EventModel, EventModelId};
// TODO: implement in terms of DTOs, as appropriate

// pub trait EventModelRepository<T: EventModel> {
//     type Error;
//
//     fn all_ids(&self) -> HashSet<EventModelId>;
//     fn by_id(&self, id: &EventModelId) -> Option<T>;
//     fn by_name(&self, id: &str) -> Option<T>;
//
//     fn put_by_id(
//         &self,
//         id: &EventModelId,
//         event_model: &T
//     ) -> Result<(), Self::Error>;
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 4;
        assert_eq!(result, 4);
    }
}
