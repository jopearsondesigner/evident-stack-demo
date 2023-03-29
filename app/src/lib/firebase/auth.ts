import { sendSignInLinkToEmail, isSignInWithEmailLink, signInWithEmailLink } from 'firebase/auth'
import { auth } from './init'

export const sendSignInLink = (email: string, redirectUrl: string) => {
  const actionCodeSettings = {
    url: redirectUrl,
    handleCodeInApp: true,
  }
  return sendSignInLinkToEmail(auth, email, actionCodeSettings)
}

export const isSignInLink = (link: string) => {
	return isSignInWithEmailLink(auth, link)
}

export const signInWithLink = (email: string, link: string) => {
	return signInWithEmailLink(auth, email, link)
}
