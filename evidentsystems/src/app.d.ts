// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
  // interface Error {}
  // interface Locals {}
  // interface PageData {}
  interface Platform {
    env: {};
    context: {
      waitUntil(promise: Promise<any>): void;
    };
    caches: CacheStorage & { default: Cache };
  }
}

declare module 'svelte-layout-aware-transitions';

declare module 'svelte-click-outside';

declare module '@lottiefiles/svelte-lottie-player';

declare module 'svelte-carousel';

declare module 'svelte-scrollto';
