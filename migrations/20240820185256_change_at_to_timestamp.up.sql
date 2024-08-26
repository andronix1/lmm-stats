delete from event_calls;
alter table event_calls alter column at type timestamptz using now();