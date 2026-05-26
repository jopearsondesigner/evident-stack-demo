import { i as initialize_decider } from '../../../../../chunks/event_model.js';
import { e as error } from '../../../../../chunks/index.js';

const ssr = false;
const load = async (event) => {
  let { session } = await event.parent();
  if (session) {
    let { decider } = await initialize_decider(void 0, session.user.id);
    return { create_model: decider.create_model };
  }
  throw error(404, { message: "not found" });
};

export { load, ssr };
