export const ssr = false;
export const prerender = true;

const fakeSupabase = {
  auth: {
    getSession: async () => ({
      data: {
        session: null
      }
    }),

    signOut: async () => ({ error: null }),

    onAuthStateChange: () => ({
      data: {
        subscription: {
          unsubscribe: () => {}
        }
      }
    })
  }
};

export const load = async () => {
  return {
    supabase: fakeSupabase,
    session: null
  };
};
