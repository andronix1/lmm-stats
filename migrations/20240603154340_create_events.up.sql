create table events(
    name text not null,
    system text not null references systems(name),
    human_name text not null,
    unique(human_name, system),
    primary key(system, name)
)