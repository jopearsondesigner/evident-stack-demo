import { P as PUBLIC_SUPABASE_URL, a as PUBLIC_SUPABASE_ANON_KEY } from './public.js';
import { createSupabaseLoadClient } from '@supabase/auth-helpers-sveltekit';

let supabase;
const init_supabase = (session, f) => {
  supabase = createSupabaseLoadClient({
    supabaseUrl: PUBLIC_SUPABASE_URL,
    supabaseKey: PUBLIC_SUPABASE_ANON_KEY,
    event: { fetch: f ? f : fetch },
    serverSession: session
  });
  return supabase;
};

export { init_supabase as i, supabase as s };
