import { browser } from "$app/environment";
import { goto } from "$app/navigation";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params, parent }) => {
  const handle_close = async () => {
    await goto(`/projects/${params.id}/design`, { noScroll: true })
  }

  if (browser) {
    const { decider } = await parent();

    try {
      let placement = await decider?.placement_by_id(params.placement);
      console.log("decider", decider, "placement", placement);
      if (placement) {
        return {
          placement,
          handle_close,
        };
      } else {
        throw "not found";
      }
    } catch (e) {
      throw error(404, `No placement found with id ${params.placement}`)
    }
  } else {
    return { handle_close }
  }
};
