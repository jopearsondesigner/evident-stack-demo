import { initializeEventModelStore } from '$lib/state';
import { browser } from '$app/environment';
import type { LayoutLoad } from './$types';
import {v4 as uuidv4} from 'uuid';

export const load: LayoutLoad = async (_event) => {
  if (browser) {
    let id = uuidv4()
    let {store, dispatch} = initializeEventModelStore(id);
    return {
      id,
      eventModel: store,
      dispatch
    };
  }
};
