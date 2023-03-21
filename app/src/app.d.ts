import type { User } from "$lib/user";

declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      user: User | null
    }
    // interface PageData {}
    // interface Platform {}
  }
}

export {};
