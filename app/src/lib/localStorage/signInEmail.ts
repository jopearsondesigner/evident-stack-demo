const KEY = 'signInEmail'

export const setSignInEmail = (email: string) => localStorage.setItem(KEY, email)

export const getSignInEmail = () => localStorage.getItem(KEY)

export const clearSignInEmail = () => localStorage.removeItem(KEY)
