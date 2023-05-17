import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async (_event) => {
  return { product: 'domain-functions' };
};
