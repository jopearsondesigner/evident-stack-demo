import { goto, invalidateAll } from "$app/navigation"

export type User = {
  id: string
  email: string
}

export const handleSignOut = function (_e: any) {
  fetch('/auth/session', { method: 'DELETE' })
    .then(() => invalidateAll())
    .then(() => goto('/'))
}
