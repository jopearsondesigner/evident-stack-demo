-- Create the bucket

insert into storage.buckets
  (id, name)
values
  ('interface-images', 'interface-images');

-- Access Policy

create or replace function auth_user_is_editor_or_better_for_interface_image_path(interface_image_path text)
returns boolean
security definer
set search_path = public
as $$
  declare
    folders text[] := storage.foldername(interface_image_path);
    model_id uuid := folders[1];
  begin
    return model_id in (select get_editor_or_better_models_for_auth_user());
  end;
$$ language plpgsql;

drop policy if exists "public download access"
  on storage.objects;

create policy "public download access"
  on storage.objects for select
  using ( bucket_id = 'interface-images' );

drop policy if exists "editors or better of models with id in path can insert images in that path"
  on storage.objects;

create policy "editors or better of models with id in path can insert images in that path"
  on storage.objects
  for insert to authenticated
  with check (
    bucket_id = 'interface-images'
    and
    (select auth_user_is_editor_or_better_for_interface_image_path("name"))
  );

drop policy if exists "editors or better of models with id in path can update images in that path"
  on storage.objects;

create policy "editors or better of models with id in path can update images in that path"
  on storage.objects
  for update to authenticated
  using (
    bucket_id = 'interface-images'
    and
    (select auth_user_is_editor_or_better_for_interface_image_path("name"))
  );

drop policy if exists "editors or better of models with id in path can delete images in that path"
  on storage.objects;

create policy "editors or better of models with id in path can delete images in that path"
  on storage.objects
  for delete to authenticated
  using (
    bucket_id = 'interface-images'
    and
    (select auth_user_is_editor_or_better_for_interface_image_path("name"))
  );
