import {
  PUBLIC_FIREBASE_PROJECT_ID,
  PUBLIC_FIREBASE_AUTH_DOMAIN,
  PUBLIC_FIREBASE_API_KEY,
  PUBLIC_FIREBASE_STORAGE_BUCKET,
  PUBLIC_FIREBASE_MESSAGE_SENDER_ID,
  PUBLIC_FIREBASE_APP_ID
} from '$env/static/public'
import { initializeApp, type FirebaseApp } from 'firebase/app'
import { getAnalytics, type Analytics } from 'firebase/analytics'
import { getAuth, sendSignInLinkToEmail, inMemoryPersistence, setPersistence, isSignInWithEmailLink, signInWithEmailLink, type Auth } from 'firebase/auth'
import { browser } from '$app/environment'

const firebaseConfig = {
  apiKey: PUBLIC_FIREBASE_API_KEY,
  authDomain: PUBLIC_FIREBASE_AUTH_DOMAIN,
  projectId: PUBLIC_FIREBASE_PROJECT_ID,
  storageBucket: PUBLIC_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: PUBLIC_FIREBASE_MESSAGE_SENDER_ID,
  appId: PUBLIC_FIREBASE_APP_ID
}

let firebase: { app: FirebaseApp, auth: Auth, analytics?: Analytics}

export const initFirebase = () => {
  const app = initializeApp(firebaseConfig);
  let auth = getAuth(app)
  setPersistence(auth, inMemoryPersistence)
  firebase = {
    app: app,
    auth: auth
  }
  if (browser) {
    firebase['analytics'] = getAnalytics(app);
  }
}

export const sendSignInLink = (email: string, redirectUrl: string) => {
  let { auth } = firebase
  const actionCodeSettings = {
    url: redirectUrl,
    handleCodeInApp: true,
  }
  return sendSignInLinkToEmail(auth, email, actionCodeSettings)
}

export const isSignInLink = (link: string) => {
  let { auth } = firebase

	return isSignInWithEmailLink(auth, link)
}

export const signInWithLink = (email: string, link: string) => {
	let { auth } = firebase

	return signInWithEmailLink(auth, email, link)
}
