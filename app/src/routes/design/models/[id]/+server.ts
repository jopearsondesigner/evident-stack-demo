import { error, json } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ request }) => {
  try {
    return json({})
  } catch (e) {
    throw error(404)
  }
}
