import type { DecodedIdToken } from 'firebase-admin/auth'
import { auth } from './init'

export const createSessionCookie = async (token: string, maxAge: number) => {
  const expiresIn = maxAge * 1000
  const sessionCookie = await auth.createSessionCookie(token, { expiresIn })
  const cookieOpts: {path: string, sameSite: 'strict', maxAge: number} =
    {path: '/', sameSite: 'strict', maxAge: maxAge}

  return {sessionCookie, cookieOpts}
}

export const verifyIdToken = (token: string): Promise<DecodedIdToken> => {
	return auth.verifyIdToken(token)
}

export const getIdTokenFromSessionCookie = (
	sessionCookie: string | null
): Promise<DecodedIdToken | null> => {
	if (!sessionCookie) return Promise.resolve(null)

	return auth.verifySessionCookie(sessionCookie, true).catch(() => null)
}
