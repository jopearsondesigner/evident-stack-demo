export const manifest = {
	appDir: "_app",
	appPath: "_app",
	assets: new Set([".DS_Store","favicon/android-chrome-192x192.png","favicon/android-chrome-512x512.png","favicon/apple-touch-icon.png","favicon/favicon-16x16.png","favicon/favicon-32x32.png","favicon/favicon.ico","favicon/site.webmanifest","images/figma-logo.svg","lottie/.DS_Store","lottie/DataAnimation.json","lottie/DatabaseAnimation.json","lottie/DeployAnimation.json","lottie/DomainFunctionsAnimation.json"]),
	mimeTypes: {".png":"image/png",".ico":"image/vnd.microsoft.icon",".webmanifest":"application/manifest+json",".svg":"image/svg+xml",".json":"application/json"},
	_: {
		client: {"start":"_app/immutable/entry/start.005f0803.js","app":"_app/immutable/entry/app.284ecbcb.js","imports":["_app/immutable/entry/start.005f0803.js","_app/immutable/chunks/index.959f4cef.js","_app/immutable/chunks/singletons.9b2e0396.js","_app/immutable/chunks/index.8aa1a1f6.js","_app/immutable/chunks/control.f5b05b5f.js","_app/immutable/entry/app.284ecbcb.js","_app/immutable/chunks/preload-helper.41c905a7.js","_app/immutable/chunks/dexie.ad0fafb0.js","_app/immutable/chunks/client.12b74c51.js","_app/immutable/chunks/_commonjsHelpers.725317a4.js","_app/immutable/chunks/index.8aa1a1f6.js","_app/immutable/chunks/index.959f4cef.js","_app/immutable/chunks/util.13123f40.js"],"stylesheets":[],"fonts":[]},
		nodes: [
			() => import('./nodes/0.js'),
			() => import('./nodes/1.js'),
			() => import('./nodes/2.js'),
			() => import('./nodes/3.js'),
			() => import('./nodes/4.js'),
			() => import('./nodes/5.js'),
			() => import('./nodes/6.js'),
			() => import('./nodes/7.js'),
			() => import('./nodes/8.js'),
			() => import('./nodes/9.js'),
			() => import('./nodes/10.js'),
			() => import('./nodes/11.js'),
			() => import('./nodes/12.js'),
			() => import('./nodes/13.js'),
			() => import('./nodes/14.js'),
			() => import('./nodes/15.js'),
			() => import('./nodes/16.js'),
			() => import('./nodes/17.js'),
			() => import('./nodes/18.js'),
			() => import('./nodes/19.js'),
			() => import('./nodes/20.js'),
			() => import('./nodes/21.js'),
			() => import('./nodes/22.js'),
			() => import('./nodes/23.js'),
			() => import('./nodes/24.js'),
			() => import('./nodes/25.js')
		],
		routes: [
			{
				id: "/(non-project)",
				pattern: /^\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 11 },
				endpoint: null
			},
			{
				id: "/(non-project)/account",
				pattern: /^\/account\/?$/,
				params: [],
				page: { layouts: [0,2,], errors: [1,,], leaf: 12 },
				endpoint: null
			},
			{
				id: "/(non-project)/auth",
				pattern: /^\/auth\/?$/,
				params: [],
				page: { layouts: [0,2,3,], errors: [1,,,], leaf: 13 },
				endpoint: null
			},
			{
				id: "/(non-project)/auth/callback",
				pattern: /^\/auth\/callback\/?$/,
				params: [],
				page: null,
				endpoint: () => import('./entries/endpoints/(non-project)/auth/callback/_server.ts.js')
			},
			{
				id: "/demo/evident-stack",
				pattern: /^\/demo\/evident-stack\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 25 },
				endpoint: null
			},
			{
				id: "/(non-project)/projects/new",
				pattern: /^\/projects\/new\/?$/,
				params: [],
				page: { layouts: [0,2,4,], errors: [1,,,], leaf: 14 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]",
				pattern: /^\/projects\/([^/]+?)\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,], errors: [1,,], leaf: 15 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/data",
				pattern: /^\/projects\/([^/]+?)\/data\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,6,], errors: [1,,,], leaf: 16 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/data/schemas/[schema]",
				pattern: /^\/projects\/([^/]+?)\/data\/schemas\/([^/]+?)\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false},{"name":"schema","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,6,], errors: [1,,,], leaf: 17 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/db",
				pattern: /^\/projects\/([^/]+?)\/db\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,7,], errors: [1,,,], leaf: 18 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/deploy",
				pattern: /^\/projects\/([^/]+?)\/deploy\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,8,], errors: [1,,,], leaf: 19 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/design",
				pattern: /^\/projects\/([^/]+?)\/design\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,9,], errors: [1,,,], leaf: 20 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/design/import",
				pattern: /^\/projects\/([^/]+?)\/design\/import\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,9,], errors: [1,,,], leaf: 21 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/design/placements/[placement]",
				pattern: /^\/projects\/([^/]+?)\/design\/placements\/([^/]+?)\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false},{"name":"placement","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,9,], errors: [1,,,], leaf: 22 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/domain-functions",
				pattern: /^\/projects\/([^/]+?)\/domain-functions\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,10,], errors: [1,,,], leaf: 23 },
				endpoint: null
			},
			{
				id: "/(project)/projects/[id]/settings/confirm-delete",
				pattern: /^\/projects\/([^/]+?)\/settings\/confirm-delete\/?$/,
				params: [{"name":"id","optional":false,"rest":false,"chained":false}],
				page: { layouts: [0,5,], errors: [1,,], leaf: 24 },
				endpoint: null
			}
		],
		matchers: async () => {
			
			return {  };
		}
	}
};
