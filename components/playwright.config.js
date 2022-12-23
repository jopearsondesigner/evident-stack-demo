/** @type {import('@playwright/test').PlaywrightTestConfig} */
const config = {
	webServer: {
		command: 'yarn dev',
		port: 5173
	},
	testDir: 'tests'
};

export default config;
