import { r as redirect } from '../../../chunks/index.js';
import { a as models_for_user } from '../../../chunks/database.js';

const load = async (event) => {
  let { session, supabase } = await event.parent();
  if (session) {
    let projects;
    try {
      const user_id = session.user.id;
      const models = await models_for_user(supabase);
      projects = models.map((m) => {
        return { user: user_id, ...m, description: m.description || "" };
      });
    } catch (e) {
      {
        projects = [];
      }
    }
    return { projects };
  }
  throw redirect(303, "/auth");
};

export { load };
