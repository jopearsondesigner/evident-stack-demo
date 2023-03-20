import { memoize } from 'lodash'
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

export const initAdmin = memoize(() => {
  const app = initializeApp(adminConfig);
  let auth = getAuth(app)
  let admin: { app: App, auth: Auth } = {
    app: app,
    auth: auth
  }
  return admin
})

export const createSessionCookie = async (token: string, maxAge: number) => {
  const expiresIn = maxAge * 1000
  const { auth } = initAdmin()
  const sessionCookie = await auth.createSessionCookie(token, { expiresIn })
  const cookieOpts: {path: string, sameSite: 'strict', maxAge: number} =
    {path: '/', sameSite: 'strict', maxAge: maxAge}

  return {sessionCookie, cookieOpts}
}

export const verifyIdToken = (token: string): Promise<DecodedIdToken> => {
	const { auth } = initAdmin()
	return auth.verifyIdToken(token)
}
