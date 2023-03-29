import { cert, initializeApp } from 'firebase-admin/app'
import { PUBLIC_FIREBASE_PROJECT_ID } from '$env/static/public'
import { FIREBASE_ADMIN_CLIENT_EMAIL, FIREBASE_ADMIN_PRIVATE_KEY } from '$env/static/private'
import { getAuth } from 'firebase-admin/auth'

export const app = initializeApp({
  credential: cert({
    projectId: PUBLIC_FIREBASE_PROJECT_ID,
    clientEmail: FIREBASE_ADMIN_CLIENT_EMAIL,
    privateKey: FIREBASE_ADMIN_PRIVATE_KEY.replace(/\\n/g, '\n')
  })
})

export const auth = getAuth(app)
