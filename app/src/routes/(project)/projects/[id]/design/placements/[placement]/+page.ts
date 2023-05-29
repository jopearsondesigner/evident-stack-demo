import { goto } from "$app/navigation";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params, parent }) => {
  const handle_close = async () => {
    await goto(`/projects/${params.id}/design`, { noScroll: true })
  }

  return {
    placement_id: params.placement,
    handle_close,
    ...await parent()
  };
};
