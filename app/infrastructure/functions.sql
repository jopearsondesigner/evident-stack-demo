-- Decider Functions

create or replace function get_teams_for_authenticated_user()
returns setof bigint
language sql
security definer
set search_path = public
stable
as $$
    select team_id
    from members
    where user_id = auth.uid()
$$;
