import type { SupabaseClient } from '@supabase/supabase-js';
import type { Database } from '$lib/supabase/database.types';

export const model_by_id = async (supabase: SupabaseClient<Database>, id: string) => {
  let result = await supabase.from("models").select().eq('id', id).limit(1);
  return result.data?.at(0);
}
