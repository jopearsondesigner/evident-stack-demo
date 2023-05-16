import { redirect } from "@sveltejs/kit";
import type { PageLoad } from "../../$types";

export const load: PageLoad = async (_event) => {
  throw redirect(308, "./design")
};
