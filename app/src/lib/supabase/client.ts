import {
  PUBLIC_SUPABASE_ANON_KEY,
  PUBLIC_SUPABASE_URL
} from '$env/static/public';
import { createSupabaseLoadClient } from '@supabase/auth-helpers-sveltekit';
import type { Session, SupabaseClient } from '@supabase/supabase-js';
import type { Database } from './database.types';

export let supabase: SupabaseClient<Database>;

export const init_supabase = (session: Session | null, f?: typeof fetch): SupabaseClient<Database> => {
  supabase = createSupabaseLoadClient({
    supabaseUrl: PUBLIC_SUPABASE_URL,
    supabaseKey: PUBLIC_SUPABASE_ANON_KEY,
    event: { fetch: f ? f : fetch },
    serverSession: session
  });
  return supabase;
};
