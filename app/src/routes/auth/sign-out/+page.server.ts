// src/routes/+page.server.ts
import { redirect } from '@sveltejs/kit';
import type { Actions } from './$types';

export const actions = {
	default: async ({ locals: { supabase, getSession } }) => {
    console.log("signing out")
		const session = await getSession();
		if (session) {
			await supabase.auth.signOut();
			throw redirect(303, '/');
		}
	}
} satisfies Actions;
