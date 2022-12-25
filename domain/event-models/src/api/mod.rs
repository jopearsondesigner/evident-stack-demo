mod grid;

use crate::types::event_model::{EventModel, EventModelId};

trait EventModelCreationContext {
    type Error;

    fn name_exists(name: &str) -> bool;
    fn create(name: &str) -> Result<EventModel, Self::Error>;
}

trait EventModelProvider {
    fn by_id(&self, id: &EventModelId) -> Option<EventModel>;
}

trait EventModelModifier: EventModelProvider {
    //     fun name(name: Name): EventModelBuilder<T>
    //     fun description(description: Description): EventModelBuilder<T>
    //
    //     fun plusInterface(`interface`: Interface): EventModelBuilder<T>
    //     fun minusInterface(interfaceId: InterfaceId): EventModelBuilder<T>
    //
    //     fun plusCommand(command: Command): EventModelBuilder<T>
    //     fun minusCommand(commandId: CommandId): EventModelBuilder<T>
    //
    //     fun plusEvent(event: Event): EventModelBuilder<T>
    //     fun minusEvent(eventId: EventId): EventModelBuilder<T>
    //
    //     fun plusReadModel(readModel: ReadModel): EventModelBuilder<T>
    //     fun minusReadModel(readModelId: ReadModelId): EventModelBuilder<T>
    //
    //     fun plusAudience(audience: Audience, index: Int): EventModelBuilder<T>
    //     fun minusAudience(audienceId: AudienceId): EventModelBuilder<T>
    //
    //     fun plusStream(stream: Stream, index: Int): EventModelBuilder<T>
    //     fun minusStream(streamId: StreamId): EventModelBuilder<T>
    //
    //     fun plusPlacement(placement: Placement): EventModelBuilder<T>
    //     fun minusPlacement(placementId: PlacementId): EventModelBuilder<T>
    //
    //     fun plusFlow(flowArrow: FlowArrow): EventModelBuilder<T>
    //     fun minusFlow(from: PlacementId, to: PlacementId): EventModelBuilder<T> =
    //         minusFlow(FlowArrow.flowId(from, to))
    //
    //     fun minusFlow(flowId: FlowId): EventModelBuilder<T>
}