import type { GridMode } from "$components/design/Grid"
import type { PageLoad } from "./$types"

export const load: PageLoad = async (_) => {
  return { mode: 'navigation' as GridMode }
}
