import '../../../../../../../../chunks/index.js';

/**
 * @template {keyof typeof client} T
 * @param {T} key
 * @returns {typeof client[T]}
 */
function client_method(key) {
	{
		if (key === 'before_navigate' || key === 'after_navigate') {
			// @ts-expect-error doesn't recognize that both keys here return void so expects a async function
			return () => {};
		} else {
			/** @type {Record<string, string>} */
			const name_lookup = {
				disable_scroll_handling: 'disableScrollHandling',
				preload_data: 'preloadData',
				preload_code: 'preloadCode',
				invalidate_all: 'invalidateAll'
			};

			return () => {
				throw new Error(`Cannot call ${name_lookup[key] ?? key}(...) on the server`);
			};
		}
	}
}

/**
 * Returns a Promise that resolves when SvelteKit navigates (or fails to navigate, in which case the promise rejects) to the specified `url`.
 * For external URLs, use `window.location = url` instead of calling `goto(url)`.
 *
 * @type {(url: string | URL, opts?: {
 *   replaceState?: boolean;
 *   noScroll?: boolean;
 *   keepFocus?: boolean;
 *   invalidateAll?: boolean;
 *   state?: any
 * }) => Promise<void>}
 * @param {string | URL} url Where to navigate to. Note that if you've set [`config.kit.paths.base`](https://kit.svelte.dev/docs/configuration#paths) and the URL is root-relative, you need to prepend the base path if you want to navigate within the app.
 * @param {Object} [opts] Options related to the navigation
 * @param {boolean} [opts.replaceState] If `true`, will replace the current `history` entry rather than creating a new one with `pushState`
 * @param {boolean} [opts.noScroll] If `true`, the browser will maintain its scroll position rather than scrolling to the top of the page after navigation
 * @param {boolean} [opts.keepFocus] If `true`, the currently focused element will retain focus after navigation. Otherwise, focus will be reset to the body
 * @param {boolean} [invalidateAll] If `true`, all `load` functions of the page will be rerun. See https://kit.svelte.dev/docs/load#rerunning-load-functions for more info on invalidation.
 * @param {any} [opts.state] The state of the new/updated history entry
 * @returns {Promise<void>}
 */
const goto = /* @__PURE__ */ client_method('goto');

const load = async ({ params, parent }) => {
  const handle_close = async () => {
    await goto(`/projects/${params.id}/design`, { noScroll: true });
  };
  {
    return {
      handle_close,
      mode: "modal"
    };
  }
};

export { load };
