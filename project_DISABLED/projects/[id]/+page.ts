import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (event) => {
  throw redirect(308, `${event.url.href}/design`)
};
