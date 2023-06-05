import { browser } from "$app/environment";
import { goto } from "$app/navigation";
import type { Placement } from "$components/design/Grid";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params, parent }) => {
  const handle_close = async () => {
    await goto(`/projects/${params.id}/design`, { noScroll: true })
  }

  if (browser) {
    const { grid } = await parent();

    try {
      let placement: Placement = await new Promise((resolve, reject) => {
        let unsubscribe = grid!.subscribe((g) => {
          if (g) {
            console.log("grid from subscription:", g)
            let p = g.placement_by_id(params.placement);
            console.log("placement from grid:", p)
            if (p) {
              resolve(p);
            } else {
              reject("placement not found!")
            }
          } else {
            reject("no grid found!")
          }
        });
        unsubscribe();
      });
      return {
        placement,
        handle_close,
      };
    } catch (e) {
      console.error("caught error", e)
      throw error(404, `No placement found with id ${params.placement}`)
    }
  } else {
    return { handle_close }
  }
};
