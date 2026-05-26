const models_for_user = async (supabase) => {
  let result = await supabase.from("models").select();
  if (result.error) {
    throw result.error;
  } else {
    return result.data;
  }
};
const model_by_id = async (supabase, id) => {
  let result = await supabase.from("models").select().eq("id", id).limit(1);
  if (result.error) {
    throw result.error;
  } else {
    return result.data?.at(0);
  }
};

export { models_for_user as a, model_by_id as m };
