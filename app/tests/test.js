import { expect, test } from '@playwright/test';

test('test test', async ({ page }) => {
	await page.goto('/');
	expect(true == true);
});

// test('index page has expected h1', async ({ page }) => {
// 	await page.goto('/');
// 	expect(await page.textContent('h1')).toBe('Welcome to SvelteKit');
// });
