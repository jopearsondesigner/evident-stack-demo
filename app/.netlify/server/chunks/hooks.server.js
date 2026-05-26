import { P as PUBLIC_SUPABASE_URL, a as PUBLIC_SUPABASE_ANON_KEY } from './public.js';
import { createSupabaseServerClient } from '@supabase/auth-helpers-sveltekit';
import { r as redirect } from './index.js';

/**
 * A helper function for sequencing multiple `handle` calls in a middleware-like manner.
 * The behavior for the `handle` options is as follows:
 * - `transformPageChunk` is applied in reverse order and merged
 * - `preload` is applied in forward order, the first option "wins" and no `preload` options after it are called
 * - `filterSerializedResponseHeaders` behaves the same as `preload`
 *
 * ```js
 * /// file: src/hooks.server.js
 * import { sequence } from '@sveltejs/kit/hooks';
 *
 * /// type: import('@sveltejs/kit').Handle
 * async function first({ event, resolve }) {
 * 	console.log('first pre-processing');
 * 	const result = await resolve(event, {
 * 		transformPageChunk: ({ html }) => {
 * 			// transforms are applied in reverse order
 * 			console.log('first transform');
 * 			return html;
 * 		},
 * 		preload: () => {
 * 			// this one wins as it's the first defined in the chain
 * 			console.log('first preload');
 * 		}
 * 	});
 * 	console.log('first post-processing');
 * 	return result;
 * }
 *
 * /// type: import('@sveltejs/kit').Handle
 * async function second({ event, resolve }) {
 * 	console.log('second pre-processing');
 * 	const result = await resolve(event, {
 * 		transformPageChunk: ({ html }) => {
 * 			console.log('second transform');
 * 			return html;
 * 		},
 * 		preload: () => {
 * 			console.log('second preload');
 * 		},
 * 		filterSerializedResponseHeaders: () => {
 * 			// this one wins as it's the first defined in the chain
 *    		console.log('second filterSerializedResponseHeaders');
 * 		}
 * 	});
 * 	console.log('second post-processing');
 * 	return result;
 * }
 *
 * export const handle = sequence(first, second);
 * ```
 *
 * The example above would print:
 *
 * ```
 * first pre-processing
 * first preload
 * second pre-processing
 * second filterSerializedResponseHeaders
 * second transform
 * first transform
 * second post-processing
 * first post-processing
 * ```
 *
 * @param {...import('@sveltejs/kit').Handle} handlers The chain of `handle` functions
 * @returns {import('@sveltejs/kit').Handle}
 */
function sequence(...handlers) {
	const length = handlers.length;
	if (!length) return ({ event, resolve }) => resolve(event);

	return ({ event, resolve }) => {
		return apply_handle(0, event, {});

		/**
		 * @param {number} i
		 * @param {import('@sveltejs/kit').RequestEvent} event
		 * @param {import('@sveltejs/kit').ResolveOptions | undefined} parent_options
		 * @returns {import('types').MaybePromise<Response>}
		 */
		function apply_handle(i, event, parent_options) {
			const handle = handlers[i];

			return handle({
				event,
				resolve: (event, options) => {
					/** @type {import('@sveltejs/kit').ResolveOptions['transformPageChunk']} */
					const transformPageChunk = async ({ html, done }) => {
						if (options?.transformPageChunk) {
							html = (await options.transformPageChunk({ html, done })) ?? '';
						}

						if (parent_options?.transformPageChunk) {
							html = (await parent_options.transformPageChunk({ html, done })) ?? '';
						}

						return html;
					};

					/** @type {import('@sveltejs/kit').ResolveOptions['filterSerializedResponseHeaders']} */
					const filterSerializedResponseHeaders =
						parent_options?.filterSerializedResponseHeaders ??
						options?.filterSerializedResponseHeaders;

					/** @type {import('@sveltejs/kit').ResolveOptions['preload']} */
					const preload = parent_options?.preload ?? options?.preload;

					return i < length - 1
						? apply_handle(i + 1, event, {
								transformPageChunk,
								filterSerializedResponseHeaders,
								preload
						  })
						: resolve(event, { transformPageChunk, filterSerializedResponseHeaders, preload });
				}
			});
		}
	};
}

const authentication = async ({ event, resolve }) => {
  event.locals.supabase = createSupabaseServerClient({
    supabaseUrl: PUBLIC_SUPABASE_URL,
    supabaseKey: PUBLIC_SUPABASE_ANON_KEY,
    event
  });
  event.locals.getSession = async () => {
    const {
      data: { session }
    } = await event.locals.supabase.auth.getSession();
    return session;
  };
  return resolve(event, {
    filterSerializedResponseHeaders(name) {
      return name === "content-range";
    }
  });
};
const authorization = async ({ event, resolve }) => {
  if (!event.url.pathname.startsWith("/auth") && !event.url.pathname.startsWith("/demo")) {
    if (!await event.locals.getSession()) {
      event.cookies.set("redirect_to", event.url.pathname, { path: "/" });
      throw redirect(303, "/auth");
    }
  }
  return resolve(event);
};
const handle = sequence(authentication, authorization);

export { handle };
