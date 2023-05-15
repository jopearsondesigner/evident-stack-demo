import { init_supabase } from '$lib/supabase/client';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch, data, depends }) => {
  depends('supabase:auth');

  const supabase = init_supabase(data.session, fetch);

  const {
    data: { session }
  } = await supabase.auth.getSession();

  return { supabase, session };
};
