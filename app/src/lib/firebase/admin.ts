import { cert, initializeApp, type App } from 'firebase-admin/app'
import { Auth, getAuth, type DecodedIdToken } from 'firebase-admin/auth'
import { PUBLIC_FIREBASE_PROJECT_ID } from '$env/static/public'
import { FIREBASE_ADMIN_CLIENT_EMAIL, FIREBASE_ADMIN_PRIVATE_KEY } from '$env/static/private'

const adminConfig = {
  credential: cert({
    projectId: PUBLIC_FIREBASE_PROJECT_ID,
    clientEmail: FIREBASE_ADMIN_CLIENT_EMAIL,
    privateKey: FIREBASE_ADMIN_PRIVATE_KEY.replace(/\\n/g, '\n')
  })
}

let admin: { app: App, auth: Auth };

export const initAdmin = () => {
  const app = initializeApp(adminConfig);
  let auth = getAuth(app)
  admin = {
    app: app,
    auth: auth
  }
}

export const createSessionCookie = async (token: string, maxAge: number) => {
  const expiresIn = maxAge * 1000
  const { auth } = admin
  const sessionCookie = await auth.createSessionCookie(token, { expiresIn })
  const cookieOpts: {path: string, sameSite: 'strict', maxAge: number} =
    {path: '/', sameSite: 'strict', maxAge: maxAge}

  return {sessionCookie, cookieOpts}
}

export const verifyIdToken = (token: string): Promise<DecodedIdToken> => {
	const { auth } = admin
	return auth.verifyIdToken(token)
}

export const getIdTokenFromSessionCookie = (
	sessionCookie: string | null
): Promise<DecodedIdToken | null> => {
	if (!sessionCookie) return Promise.resolve(null)

	const { auth } = admin

	return auth.verifySessionCookie(sessionCookie, true).catch(() => null)
}
