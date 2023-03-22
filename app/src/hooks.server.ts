import { SESSION_COOKIE_NAME } from "$lib/constants";
import { getIdTokenFromSessionCookie } from "$lib/firebase/admin";
import { redirect, type Handle } from "@sveltejs/kit";
import { sequence } from "@sveltejs/kit/hooks";

const authentication = (async ({ event, resolve }) => {
  const { cookies, locals } = event

  const sessionCookie = cookies.get(SESSION_COOKIE_NAME) || null

  const token = sessionCookie ? await getIdTokenFromSessionCookie(sessionCookie) : null

  locals.user = token ? {id: token.sub, email: token.email} : null

  return resolve(event)
}) satisfies Handle;

const authorization = (async ({ event, resolve }) => {
  if (!event.url.pathname.startsWith('/auth')) {
    if (!event.locals.user) {
      throw redirect(303, '/auth/sign-in')
    }
  }
  return resolve(event)
}) satisfies Handle;

export const handle: Handle = sequence(
  authentication,
  authorization
);
