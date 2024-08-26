delete from event_calls;
alter table event_calls alter column at type time with time zone using now();