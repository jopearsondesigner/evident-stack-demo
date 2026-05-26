import { i as init_supabase } from '../../chunks/client.js';

const load = async ({ fetch, data, depends }) => {
  depends("supabase:auth");
  const supabase = init_supabase(data.session, fetch);
  const {
    data: { session }
  } = await supabase.auth.getSession();
  return { supabase, session };
};

export { load };
