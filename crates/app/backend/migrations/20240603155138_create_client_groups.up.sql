create table client_groups(
    id uuid not null primary key default gen_random_uuid(),
    name text not null unique
)