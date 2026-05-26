import { r as redirect } from '../../../../../chunks/index.js';

const load = async (event) => {
  throw redirect(308, `${event.url.href}/design`);
};

export { load };
