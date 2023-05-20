/// <reference types="@sveltejs/kit" />

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
// and what to do when importing types
declare namespace App {
  // interface Error {}
  // interface Locals {}
  // interface PageData {}
  // interface Platform {}
}

interface HTMLOrSVGElement {
  autofocus: boolean;
  readonly dataset: DOMStringMap;
  nonce?: string;
  tabIndex: number;
  blur(): void;
  focus(options?: FocusOptions): void;
}

declare module 'lottie-web/build/player/lottie';
