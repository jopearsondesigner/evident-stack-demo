import { dev } from "$app/environment"

export const debug = (...args: any[]) => {
  if (dev) {
    console.debug(...args);
  }
};

export const parseIntOr = (int_str: string | null | undefined, or: number) => {
  if (int_str) {
    try {
      return parseInt(int_str);
    } catch(e) {
      return or;
    }
  } else {
    return or;
  }
};
