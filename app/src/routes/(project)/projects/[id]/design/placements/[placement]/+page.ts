import { browser } from '$app/environment';
import { goto } from '$app/navigation';
import type { GridMode } from '$components/design/Grid';
import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent }) => {
  const handle_close = async () => {
    await goto(`/projects/${params.id}/design`, { noScroll: true });
  };

  if (browser) {
    const { decider } = await parent();

    try {
      let placement = await decider?.placement_by_id(params.placement);
      if (placement) {
        return {
          mode: 'modal' as GridMode,
          placement,
          handle_close
        };
      } else {
        throw 'not found';
      }
    } catch (e) {
      throw error(404, `No placement found with id ${params.placement}`);
    }
  } else {
    return {
      handle_close,
      mode: 'modal' as GridMode
    };
  }
};
