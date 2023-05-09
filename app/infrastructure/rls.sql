-- RLS Policies

create or replace function get_all_models_for_auth_user()
returns setof uuid
security definer
set search_path = public
as $$
  select model
  from model_collaborators
  where "user" = auth.uid()
$$ language sql;

create or replace function get_editor_or_better_models_for_auth_user()
returns setof uuid
security definer
set search_path = public
as $$
  select model
  from model_collaborators
  where "user" = auth.uid()
    and (model_collaborators.role = 'editor'
         or model_collaborators.role = 'admin'
         or model_collaborators.role = 'owner')
$$ language sql;

create or replace function get_admin_or_better_models_for_auth_user()
returns setof uuid
security definer
set search_path = public
as $$
  select model
  from model_collaborators
  where "user" = auth.uid()
    and (model_collaborators.role = 'admin'
         or model_collaborators.role = 'owner')
$$ language sql;

create or replace function get_models_owned_by_auth_user()
returns setof uuid
security definer
set search_path = public
as $$
  select model
  from model_collaborators
  where "user" = auth.uid() and model_collaborators.role = 'owner'
$$ language sql;

alter table public.models enable row level security;

drop policy if exists "Authenticated users can create a model"
  on public.models;

create policy "Authenticated users can create a model"
  on public.models
  for insert to authenticated
  with check (
    creator = auth.uid()
  );

drop policy if exists "Model creators can see their own models"
  on public.models;

create policy "Model creators can see their own models"
  on public.models
  for select to authenticated
  using (
    creator = auth.uid()
  );

drop policy if exists "All granted roles can view a model"
  on public.models;

create policy "All granted roles can view a model"
  on public.models
  for select to authenticated
  using (
    id in (
      select get_all_models_for_auth_user()
    )
  );

drop policy if exists "Editors or better can update a model"
  on public.models;

create policy "Editors or better can update a model"
  on public.models
  for update to authenticated
  using (
    id in (
      select get_editor_or_better_models_for_auth_user()
    )
  )
  with check (
    -- Ensure the creator doesn't get overwritten,
    -- since creator can grant new collaborator roles!
    creator = null
  );

drop policy if exists "Admins or better can delete a model"
  on public.models;

create policy "Admins or better can delete a model"
  on public.models
  for delete to authenticated
  using (
    id in (
      select get_admin_or_better_models_for_auth_user()
    )
  );

alter table public.model_events enable row level security;

drop policy if exists "All granted roles can view model events"
  on public.model_events;

create policy "All granted roles can view model events"
  on public.model_events
  for select to authenticated
  using (
    subject in (
      select get_all_models_for_auth_user()
    )
  );

drop policy if exists "Editors or better can insert model events"
  on public.model_events;

create policy "Editors or better can insert model events"
  on public.model_events
  for insert to authenticated
  with check (
    subject in (
      select get_editor_or_better_models_for_auth_user()
    )
  );

alter table public.model_patches enable row level security;

drop policy if exists "All granted roles can view model patches"
  on public.model_patches;

create policy "All granted roles can view model patches"
  on public.model_patches
  for select to authenticated
  using (
    model in (
      select get_editor_or_better_models_for_auth_user()
    )
  );

drop policy if exists "Editors or better can insert model patches"
  on public.model_patches;

create policy "Editors or better can insert model patches"
  on public.model_patches
  for insert to authenticated
  with check (
    model in (
      select get_editor_or_better_models_for_auth_user()
    )
  );

alter table public.model_collaborator_invitations enable row level security;

drop policy if exists "Model collaborators can view collaborator invites"
  on public.model_collaborator_invitations;

create policy "Model collaborators can view collaborator invites"
  on public.model_collaborator_invitations
  for select to authenticated
  using (
    model in (
      select get_all_models_for_auth_user()
    )
  );

drop policy if exists "Admins or owners can invite collaborators"
  on public.model_collaborator_invitations;

create policy "Admins or owners can invite collaborators"
  on public.model_collaborator_invitations
  for insert to authenticated
  with check (
    (model in (
      select get_admin_or_better_models_for_auth_user()
    ) and (role = 'viewer' or role = 'editor' or role = 'admin'))
    or
    (model in (
      select get_models_owned_by_auth_user()
    ))
  );

-- TODO: invitors can delete their own invitations
-- TODO: users with invited email address can delete invitations (i.e. as part of the accept invitation process)

alter table public.model_collaborators enable row level security;

drop policy if exists "Model collaborators can view other collaborators"
  on public.model_collaborators;

create policy "Model collaborators can view other collaborators"
  on public.model_collaborators
  for select to authenticated
  using (
    model in (
      select get_all_models_for_auth_user()
    )
  );

drop policy if exists "Model creator can grant themselves access as and owner collaborator"
  on public.model_collaborators;

create policy "Model creator can grant themselves access as and owner collaborator"
  on public.model_collaborators
  for insert to authenticated
  with check (
    auth.uid() in (select "creator" from public.models where models.id = model limit 1)
    and "user" = auth.uid()
    and "grantor" = auth.uid()
    and role = 'owner'
  );

drop policy if exists "Admins or owners can grant access to other collaborators"
  on public.model_collaborators;

create policy "Admins or owners can grant access to other collaborators"
  on public.model_collaborators
  for insert to authenticated
  with check (
    (model in (
      select get_admin_or_better_models_for_auth_user()
    ) and (role = 'viewer' or role = 'editor' or role = 'admin'))
    or
    (model in (select get_models_owned_by_auth_user()))
  );

drop policy if exists "Admins or owners can revoke access to other collaborators"
  on public.model_collaborators;

create policy "Admins or owners can revoke access to other collaborators"
  on public.model_collaborators
  for delete to authenticated
  using (
    (model in (
      select get_admin_or_better_models_for_auth_user()
    ) and (role = 'viewer' or role = 'editor' or role = 'admin'))
    or
    (model in (select get_models_owned_by_auth_user()))
  );
