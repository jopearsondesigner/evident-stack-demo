import {
  PUBLIC_SUPABASE_URL,
  PUBLIC_SUPABASE_ANON_KEY
} from '$env/static/public';
import { createSupabaseServerClient } from '@supabase/auth-helpers-sveltekit';
import { redirect, type Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

const authentication = (async ({ event, resolve }) => {
  event.locals.supabase = createSupabaseServerClient({
    supabaseUrl: PUBLIC_SUPABASE_URL,
    supabaseKey: PUBLIC_SUPABASE_ANON_KEY,
    event
  });

  event.locals.getSession = async () => {
    const {
      data: { session }
    } = await event.locals.supabase.auth.getSession();
    return session;
  };

  return resolve(event, {
    filterSerializedResponseHeaders(name) {
      return name === 'content-range';
    }
  });
}) satisfies Handle;

const authorization = (async ({ event, resolve }) => {
  if (!event.url.pathname.startsWith('/auth')) {
    if (!(await event.locals.getSession())) {
      event.cookies.set('redirect_to', event.url.pathname, { path: '/' });
      throw redirect(303, '/auth');
    }
  }
  return resolve(event);
}) satisfies Handle;

export const handle: Handle = sequence(
  authentication,
  authorization,
);
