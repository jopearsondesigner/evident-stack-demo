import { redirect } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ cookies, url, locals: { supabase } }) => {
  const code = url.searchParams.get('code');
  const redirect_to = cookies.get('redirect_to');
  cookies.delete('redirect_to');

  if (code) {
    await supabase.auth.exchangeCodeForSession(code);
  }

  throw redirect(303, redirect_to || '/');
};
