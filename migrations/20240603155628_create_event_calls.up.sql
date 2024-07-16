create table event_calls(
    system text not null,
    event_name text not null,
    from_client uuid not null references clients(id),
    at time with time zone not null default now(),
    primary key (system, event_name, from_client, at),
    foreign key (system, event_name) references events
)