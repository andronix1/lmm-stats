create table systems(
    name text not null primary key,
    human_name text not null unique,
    owner uuid not null references users(id),
    active boolean not null default true,
    secret text
)