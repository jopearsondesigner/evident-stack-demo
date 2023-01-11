import { EventModelStateManager, EventModelCreationContext } from "editor-state";
import { readable, derived } from 'svelte/store';

const initializeEventModelCreationContext = function() {
  let creationContext = new EventModelCreationContext();
  return {
    dispatch: (command) => { creationContext.dispatch(command) }
  };
}

const initializeEventModelStore = function(id) {
  let manager = new EventModelStateManager(id);
  let store = readable(null, set => {
    manager.initialize(set);

    return () => console.debug("Unsubscribed last subscriber to Event Model", id);
  });

  return {
    store,
    dispatch: (command) => { manager.dispatch(command) }
  };
}

export { initializeEventModelStore, initializeEventModelCreationContext };
