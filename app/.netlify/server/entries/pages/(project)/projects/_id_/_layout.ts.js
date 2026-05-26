import { i as initialize_decider } from "../../../../../chunks/event_model.js";
import { e as error } from "../../../../../chunks/index.js";
import { m as model_by_id } from "../../../../../chunks/database.js";
let load, ssr;
let __tla = (async () => {
  ssr = false;
  load = async (event) => {
    let { session, supabase } = await event.parent();
    const model_id = event.params.id;
    if (session) {
      const { model_by_id: model_by_id$1 } = await import("../../../../../chunks/event_model.js").then((n) => n.d);
      const local_model = await model_by_id$1(session.user.id, model_id);
      if (local_model) {
        let { grid, decider } = await initialize_decider(model_id, session.user.id);
        return {
          grid,
          decider,
          session
        };
      } else {
        const remote_model = await model_by_id(supabase, model_id);
        if (remote_model) {
          let { grid, decider } = await initialize_decider(model_id, session.user.id);
          return {
            grid,
            decider,
            session
          };
        }
      }
    }
    throw error(404, {
      message: "not found"
    });
  };
})();
export {
  __tla,
  load,
  ssr
};
