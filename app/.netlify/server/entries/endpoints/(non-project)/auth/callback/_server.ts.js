import { r as redirect } from '../../../../../chunks/index.js';

const GET = async ({ cookies, url, locals: { supabase } }) => {
  const code = url.searchParams.get("code");
  const redirect_to = cookies.get("redirect_to");
  cookies.delete("redirect_to");
  if (code) {
    await supabase.auth.exchangeCodeForSession(code);
  }
  throw redirect(303, redirect_to || "/");
};

export { GET };
