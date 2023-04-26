import { getTokenFromGCPServiceAccount } from '@sagi.io/workers-jwt'
import { FlarebaseAuth } from '@marplex/flarebase-auth'
import { PUBLIC_FIREBASE_PROJECT_ID, PUBLIC_FIREBASE_API_KEY } from '$env/static/public'
import { FIREBASE_ADMIN_CLIENT_EMAIL, FIREBASE_ADMIN_PRIVATE_KEY } from '$env/static/private'

const config = {
  apiKey: PUBLIC_FIREBASE_API_KEY,
  projectId: PUBLIC_FIREBASE_PROJECT_ID,
  privateKey: FIREBASE_ADMIN_PRIVATE_KEY.replace(/\\n/g, '\n'),
  serviceAccountEmail: FIREBASE_ADMIN_CLIENT_EMAIL,
}

export const auth = new FlarebaseAuth(config)

const aud = `https://firestore.googleapis.com/google.firestore.v1.Firestore`
const serviceAccountJSON = {
  client_email: FIREBASE_ADMIN_CLIENT_EMAIL,
  private_key: FIREBASE_ADMIN_PRIVATE_KEY.replace(/\\n/g, '\n')
}
const projectId = PUBLIC_FIREBASE_PROJECT_ID

export const firestore = async () => {
  let token = await getTokenFromGCPServiceAccount({ serviceAccountJSON, aud} )
  return {
    headers: { Accept: 'application/json', Authorization: `Bearer ${token}` },
    patches_url: (id: string) => `https://firestore.googleapis.com/v1/projects/${projectId}/databases/(default)/documents/models/${id}/patches`,
  }
};
