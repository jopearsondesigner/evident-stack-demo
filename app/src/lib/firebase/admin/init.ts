import { Firestore } from '@google-cloud/firestore'
import { FlarebaseAuth } from '@marplex/flarebase-auth'
import { PUBLIC_FIREBASE_PROJECT_ID, PUBLIC_FIREBASE_API_KEY } from '$env/static/public'
import { FIREBASE_ADMIN_CLIENT_EMAIL, FIREBASE_ADMIN_PRIVATE_KEY } from '$env/static/private'

const config = {
  apiKey: PUBLIC_FIREBASE_API_KEY ,
  projectId: PUBLIC_FIREBASE_PROJECT_ID,
  privateKey: FIREBASE_ADMIN_PRIVATE_KEY.replace(/\\n/g, '\n'),
  serviceAccountEmail: FIREBASE_ADMIN_CLIENT_EMAIL,
}

export const auth = new FlarebaseAuth(config);

export const firestore = new Firestore(config);
