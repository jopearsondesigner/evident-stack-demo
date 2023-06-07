import type { SupabaseClient } from '@supabase/supabase-js';
import type { Database } from '$lib/supabase/database.types';

export const models_for_user = async (supabase: SupabaseClient<Database>) => {
  let result = await supabase.from("models").select();
  if (result.error) {
    throw result.error
  } else {
    return result.data;
  }
}

export const model_by_id = async (supabase: SupabaseClient<Database>, id: string) => {
  let result = await supabase.from("models").select().eq('id', id).limit(1);
  return result.data?.at(0);
}
