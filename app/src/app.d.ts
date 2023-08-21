import { SupabaseClient, Session } from '@supabase/supabase-js';
import { Database } from '$lib/supabase/database.types';
declare module 'svelte-layout-aware-transitions';
declare module '@codemirror/theme-one-dark';

declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      supabase: SupabaseClient<Database>;
      getSession(): Promise<Session | null>;
    }
    // interface Platform {}
  }
}

export {};
