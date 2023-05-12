-- Decider Functions

create or replace function create_model(model_id uuid, model_name text, model_description text)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    user_id uuid := auth.uid();
  begin
    insert into public.models (id, name, description, creator)
      values (model_id, model_name, model_description, user_id)
      on conflict do nothing;
    if not found then
      rollback;
    end if;

    insert into public.model_collaborators ("user", model, role, grantor)
      values (user_id, model_id, 'owner', user_id);

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'created', model_id, user_id,
              jsonb_build_object('id', model_id, 'name', model_name, 'description', model_description));

    return event_id;
  exception
    when transaction_rollback then
      return null;
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
    if not found then
      rollback;
    end if;

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'updated', model_id, user_id,
              jsonb_build_object('name', model_name, 'description', model_description));

    return event_id;
  exception
    when transaction_rollback then
      return null;
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
    if not found then
      rollback;
    end if;

    return event_id;
  exception
    when transaction_rollback then
      return null;
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
      values (patch_id, model_id, patch_data)
      on conflict do nothing;
    if not found then
      rollback;
    end if;

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'patched', model_id, user_id,
              jsonb_build_object('patch_id', patch_id));

    return event_id;
  exception
    when transaction_rollback then
      return null;
    when foreign_key_violation then
      return null;
  end;
$$ language plpgsql;

-- TODO: from the [ISyncProtocol remarks](https://dexie.org/docs/Syncable/Dexie.Syncable.ISyncProtocol#remarks)
--   "The implementation must not fail if trying to create an object with the same key twice,
--    or delete an object with a key that does not exist."
create or replace function apply_client_changes(changes json)
returns void
as $$
  declare
    user_id uuid := auth.uid();
  begin
    perform create_model((model_insertion ->> 'id')::uuid,
                        model_insertion ->> 'name',
                        model_insertion ->> 'description')
      from json_array_elements(changes -> 'model_insertions') as model_insertion;
    perform update_model((model_update ->> 'id')::uuid, model_update ->> 'name', model_update ->> 'description')
      from json_array_elements(changes -> 'model_updates') as model_update;
    perform append_patch((patch ->> 'model')::uuid, (patch ->> 'id')::uuid, patch ->> 'data')
      from json_array_elements(changes -> 'patch_insertions') as patch;
    perform delete_model(model_id::uuid)
      from json_array_elements_text(changes -> 'model_deletions') as model_id;
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
--       values (event_id, 'collaborator_invited', model_id, invitor_id,
--               jsonb_build_object('invitation_id', invitation_id, 'invitee_email', invitee_email, 'role', role));

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
--       values (event_id, 'collaborator_role_granted', model_id, invitor_id,
--               jsonb_build_object('invitation_id', invitation_id, 'invitee_email', invitee_email, 'role', role));

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
      values (event_id, 'collaborator_role_granted', model_id, grantor_id,
              jsonb_build_object('grantor', grantor_id, 'grantee', grantee_id, 'role', role));

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
      values (event_id, 'collaborator_role_revoked', model_id, revoker_id,
              jsonb_build_object('revoker', revoker_id, 'revokee', revokee_id));

    return event_id;
  end;
$$ language plpgsql;

create or replace function snapshot_model(model_id uuid, model_data text, as_of_event uuid)
returns uuid
as $$
  declare
    event_id uuid := gen_random_uuid();
    patch_id uuid := gen_random_uuid();
    user_id uuid := uuid_nil();
  begin
    insert into public.model_patches (id, model, data)
      values (patch_id, model_id, model_data);

    insert into public.model_events (id, type, subject, "user", data)
      values (event_id, 'snapshotted', model_id, user_id,
              jsonb_build_object(
                'patch_id', patch_id,
                'obsolete_patch_ids', (select jsonb_agg((model_events.data ->> 'patch_id')::uuid)
                                       from model_events
                                       where model_events.subject = model_id
                                         and (model_events.data ->> 'patch_id')::uuid is not null
                                         and sequence <= (select sequence
                                                          from public.model_events
                                                          where id = as_of_event))
                )
              );

    return event_id;
  end;
$$ language plpgsql;

-- Read Models

create or replace function model_events_since(model_id uuid, starting_event_id uuid)
returns table("id" uuid,
              "type" event_type,
              "subject" uuid,
              "user" uuid,
              "data" jsonb,
              "patch_data" text)
as $$
   select model_events.id,
          model_events.type,
          model_events.subject,
          model_events.user,
          model_events.data,
          model_patches.data as patch_data
   from public.model_events
   left join public.model_patches on model_patches.id = (model_events.data ->> 'patch_id')::uuid
   where model_events.subject = model_id
     and sequence > COALESCE((select sequence
                              from public.model_events
                              where id = starting_event_id), 0)
   order by sequence asc;
$$ language sql stable;
