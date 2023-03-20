import { createSessionCookie, verifyIdToken } from '$lib/firebase/admin';
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

const ONE_WEEK_IN_SECONDS = 7 * 24 * 60 * 60

export const POST = (async ({ url, request, cookies, ...rest }) => {
  const authHeader = request.headers.get('Authorization') || ''
  const [scheme, token] = authHeader.split(' ')
  if (scheme !== 'Bearer' || !token) {
    throw error(404) // Return 404 instead of 401, to deter brute-force
  }
  try {
    const { sub, email } = await verifyIdToken(token)
    const { sessionCookie, cookieOpts } = await createSessionCookie(token, ONE_WEEK_IN_SECONDS)

    const user = { id: sub, email }
    cookies.set('session', sessionCookie, cookieOpts)
    return json(user)
  } catch {
    throw error(404) // Return 404 instead of 401, to deter brute-force
  }
}) satisfies RequestHandler;

// export const DELETE: RequestHandler = (async (event) => {});
