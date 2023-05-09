-- RLS Policies

alter table public.models enable row level security;

drop policy if exists "All granted roles can view a model" on public.models;

create policy "All granted roles can view a model"
  on public.models
  for select using (
    auth.uid() in (
      select "user"
      from model_collaborators
      where model_collaborators.model = models.id
    )
  );

drop policy if exists "Editors or better can update a model" on public.models;

create policy "Editors or better can update a model"
  on public.models
  for update using (
    auth.uid() in (
      select "user"
      from model_collaborators
      where model_collaborators.model = models.id
      and (model_collaborators.role = 'editor'
           or model_collaborators.role = 'admin'
           or model_collaborators.role = 'owner')
    )
  );

alter table public.model_events enable row level security;

drop policy if exists "All granted roles can view model events" on public.model_events;

create policy "All granted roles can view model events"
  on public.model_events
  for select using (
    auth.uid() in (
      select "user"
      from model_collaborators
      where model_collaborators.model = model_events.subject
    )
  );

alter table public.model_patches enable row level security;

drop policy if exists "All granted roles can view model patches" on public.model_patches;

create policy "All granted roles can view model patches"
  on public.model_patches
  for select using (
    auth.uid() in (
      select "user"
      from model_collaborators
      where model_collaborators.model = model_patches.model
      and (model_collaborators.role = 'editor'
           or model_collaborators.role = 'admin'
           or model_collaborators.role = 'owner')
    )
  );

alter table public.model_collaborator_invitations enable row level security;
alter table public.model_collaborators enable row level security;
