create extension pgcrypto;

create type user_role as enum('superuser', 'developer', 'viewer');

create table users (
    id uuid primary key default gen_random_uuid(), 
    login text unique not null,
    encrypted_password text not null,
    revision integer not null default 0,
    role user_role not null
);