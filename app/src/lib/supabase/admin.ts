import { createClient } from '@supabase/supabase-js'
import { PUBLIC_SUPABASE_URL } from '$env/static/public'
import { SUPABASE_SERVICE_KEY } from '$env/static/private'

// TODO: use Svelte-provided `fetch`: https://supabase.com/docs/reference/javascript/initializing
export const supabase = createClient(PUBLIC_SUPABASE_URL, SUPABASE_SERVICE_KEY)
