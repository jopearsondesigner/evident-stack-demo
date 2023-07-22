import { expect, test } from '@playwright/test';

test('true == true', async ({ page }) => {
	await page.goto('/');
	expect(true == true);
});
