import { dev } from "$app/environment"

export const debug = (...args: any[]) => {
  if (dev) {
    console.debug(...args);
  }
}
