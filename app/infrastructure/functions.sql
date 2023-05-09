-- Decider Functions

create or replace function create_model(model_id uuid, model_name text, model_description text)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    user_id uuid := auth.uid();
  begin
    insert into public.models (id, name, description, creator)
      values (model_id, model_name, model_description, user_id);

    insert into public.model_collaborators ("user", model, role, grantor)
      values (user_id, model_id, 'owner', user_id);

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'created', model_id, user_id, jsonb_build_object('id', model_id, 'name', model_name, 'description', model_description));

    return event_id;
  end;
$$ language plpgsql;

create or replace function update_model(model_id uuid, model_name text, model_description text)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    user_id uuid := auth.uid();
  begin
    update public.models set name = model_name, description = model_description
      where id = model_id;

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'updated', model_id, user_id,
              jsonb_build_object('name', model_name, 'description', model_description));

    return event_id;
  end;
$$ language plpgsql;

create or replace function delete_model(model_id uuid)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    user_id uuid := auth.uid();
  begin
    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'deleted', model_id, user_id, null);

    delete from public.models where id = model_id;

    return event_id;
  end;
$$ language plpgsql;

create or replace function append_patch(model_id uuid, patch_id uuid, patch_data text)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    user_id uuid := auth.uid();
  begin
    insert into public.model_patches (id, model, data)
      values (patch_id, model_id, patch_data);

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'patched', model_id, user_id, jsonb_build_object('patch_id', patch_id));

    return event_id;
  end;
$$ language plpgsql;

create or replace function apply_client_changes(changes json)
returns uuid
as $$
  declare
    user_id uuid := auth.uid();
  begin
    select create_model(model_insertion -> 'id', model_insertion -> 'name', model_insertion -> 'description')
      from json_array_elements(changes -> 'model_insertions') as model_insertion;
    select update_model(model_update -> 'id', model_update -> 'name', model_update -> 'description')
      from json_array_elements(changes -> 'model_updates') as model_update;
    select append_patch(patch -> 'model', patch -> 'id', patch -> 'data')
      from json_array_elements(changes -> 'patch_insertions') as patch;
    select delete_model(model_id)
      from json_array_elements(changes -> 'model_deletions') as model_id;
  end;
$$ language plpgsql;

-- create or replace function invite_collaborator(invitor_id uuid, model_id uuid, invitee_email text, role role)
-- returns uuid
-- as $$
--   declare
--     event_id uuid := gen_random_uuid();
--     invitation_id uuid := gen_random_uuid();
--   begin
--     insert into public.model_collaborator_invitations (id, model, invitor, invitee_email, role)
--       values (invitation_id, model_id, invitor_id, invitee_email, role);

--     insert into public.model_events (id, type, subject, "user", data)
--       values (event_id, 'collaborator_invited', model_id, invitor_id, jsonb_build_object('invitation_id', invitation_id, 'invitee_email', invitee_email, 'role', role));

--     return event_id;
--   end;
-- $$ language plpgsql;

-- create or replace function accept_invitation(user_id uuid, invitation_id uuid)
-- returns uuid
-- as $$
--   declare
--     event_id uuid := gen_random_uuid();
--   begin
--     insert into public.model_collaborator_invitations (id, model, invitor, invitee_email, role)
--       values (invitation_id, model_id, invitor_id, invitee_email, role);

--     insert into public.model_events (id, type, subject, "user", data)
--       values (event_id, 'collaborator_role_granted', model_id, invitor_id, jsonb_build_object('invitation_id', invitation_id, 'invitee_email', invitee_email, 'role', role));

--     return event_id;
--   end;
-- $$ language plpgsql;

create or replace function grant_collaborator_role(model_id uuid, grantee_id uuid, role role)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    grantor_id uuid := auth.uid();
  begin
    insert into public.model_collaborators ("user", model, role, grantor)
      values (grantee_id, model_id, role, grantor_id);

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'collaborator_role_granted', model_id, grantor_id, jsonb_build_object('grantor', grantor_id, 'grantee', grantee_id, 'role', role));

    return event_id;
  end;
$$ language plpgsql;

create or replace function revoke_collaborator_role(model_id uuid, revokee_id uuid)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    revoker_id uuid := auth.uid();
  begin
    delete from public.model_collaborators where "user" = revokee_id and model = model_id;

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'collaborator_role_revoked', model_id, revoker_id, jsonb_build_object('revoker', revoker_id, 'revokee', revokee_id));

    return event_id;
  end;
$$ language plpgsql;

-- TODO: fail if patch events have moved on?
create or replace function snapshot_model(model_id uuid, model_data text)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    patch_id uuid := gen_random_uuid();
    user_id uuid := uuid_nil();
  begin
    insert into public.model_patches (id, model, data)
      values (patch_id, model_id, model_data);

    -- TODO: delete/tombstone now-obsolete patches?

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'snapshotted', model_id, user_id, jsonb_build_object('patch_id', patch_id));

    return event_id;
  end;
$$ language plpgsql;
