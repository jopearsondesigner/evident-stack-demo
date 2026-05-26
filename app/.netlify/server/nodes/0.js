import * as universal from '../entries/pages/_layout.ts.js';
import * as server from '../entries/pages/_layout.server.ts.js';

export const index = 0;
export const component = async () => (await import('../entries/pages/_layout.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+layout.ts";
export { server };
export const server_id = "src/routes/+layout.server.ts";
export const imports = ["_app/immutable/nodes/0.9ff0209d.js","_app/immutable/chunks/client.12b74c51.js","_app/immutable/chunks/_commonjsHelpers.725317a4.js","_app/immutable/chunks/preload-helper.41c905a7.js","_app/immutable/chunks/index.959f4cef.js","_app/immutable/chunks/navigation.9babf070.js","_app/immutable/chunks/singletons.9b2e0396.js","_app/immutable/chunks/index.8aa1a1f6.js"];
export const stylesheets = ["_app/immutable/assets/0.ec26be64.css"];
export const fonts = [];
