import { ONE_WEEK_IN_SECONDS, SESSION_COOKIE_NAME } from '$lib/constants';
import { createSessionCookie, verifyIdToken } from '$lib/firebase/admin';
import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request, cookies }) => {
  const authHeader = request.headers.get('Authorization') || ''
  const [scheme, token] = authHeader.split(' ')
  if (scheme !== 'Bearer' || !token) {
    throw error(404) // Return 404 instead of 401, to deter brute-force
  }
  try {
    const { sub, email } = await verifyIdToken(token)
    const { sessionCookie, cookieOpts } = await createSessionCookie(token, ONE_WEEK_IN_SECONDS)

    const user = { id: sub, email }
    cookies.set(SESSION_COOKIE_NAME, sessionCookie, cookieOpts)
    return json(user)
  } catch {
    throw error(404) // Return 404 instead of 401, to deter brute-force
  }
}

export const DELETE: RequestHandler = async ({ cookies }) => {
  cookies.delete(SESSION_COOKIE_NAME, {path: '/', sameSite: 'strict', maxAge: 0})
  return json({})
}
