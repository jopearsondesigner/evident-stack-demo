import { browser } from '$app/environment';
import { isSignInLink } from '$lib/firebase/client';
import { getSignInEmail } from '$lib/localStorage/signInEmail';
import type { PageLoad } from './$types';

export const load = (({ url }) => {
  return {
    isSignInLink: isSignInLink(url.href),
    signInEmail: browser ? getSignInEmail() : null
  }
}) satisfies PageLoad;
