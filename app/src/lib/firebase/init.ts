import { browser } from '$app/environment'
import {
  PUBLIC_FIREBASE_PROJECT_ID,
  PUBLIC_FIREBASE_AUTH_DOMAIN,
  PUBLIC_FIREBASE_API_KEY,
  PUBLIC_FIREBASE_STORAGE_BUCKET,
  PUBLIC_FIREBASE_MESSAGE_SENDER_ID,
  PUBLIC_FIREBASE_APP_ID
} from '$env/static/public'
import { getAnalytics, type Analytics } from 'firebase/analytics'
import { initializeApp } from 'firebase/app'
import { getAuth, inMemoryPersistence, setPersistence } from 'firebase/auth'

export const firebase = initializeApp({
  apiKey: PUBLIC_FIREBASE_API_KEY,
  authDomain: PUBLIC_FIREBASE_AUTH_DOMAIN,
  projectId: PUBLIC_FIREBASE_PROJECT_ID,
  storageBucket: PUBLIC_FIREBASE_STORAGE_BUCKET,
  messagingSenderId: PUBLIC_FIREBASE_MESSAGE_SENDER_ID,
  appId: PUBLIC_FIREBASE_APP_ID
})

export const auth = getAuth(firebase)
setPersistence(auth, inMemoryPersistence)

export let analytics: Analytics;

if (browser) {
  analytics = getAnalytics(firebase)
}
