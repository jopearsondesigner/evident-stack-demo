import { t as getContext } from './index3.js';

/**
 * A function that returns all of the contextual stores. On the server, this must be called during component initialization.
 * Only use this if you need to defer store subscription until after the component has mounted, for some reason.
 */
const getStores = () => {
	const stores = getContext('__svelte__');

	return {
		/** @type {typeof page} */
		page: {
			subscribe: stores.page.subscribe
		},
		/** @type {typeof navigating} */
		navigating: {
			subscribe: stores.navigating.subscribe
		},
		/** @type {typeof updated} */
		updated: stores.updated
	};
};

/**
 * A readable store whose value contains page data.
 *
 * On the server, this store can only be subscribed to during component initialization. In the browser, it can be subscribed to at any time.
 *
 * @type {import('svelte/store').Readable<import('@sveltejs/kit').Page>}
 */
const page = {
	subscribe(fn) {
		const store = getStores().page;
		return store.subscribe(fn);
	}
};

export { page as p };
