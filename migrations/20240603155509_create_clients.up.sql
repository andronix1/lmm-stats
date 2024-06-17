create table clients(
    id uuid primary key default gen_random_uuid(),
    name text not null unique,
    group_id uuid not null references client_groups(id) on delete cascade
)