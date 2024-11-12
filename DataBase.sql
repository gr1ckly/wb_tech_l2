create table events(
  id BIGSERIAL not null, 
  start_time TIMESTAMP with TIME ZONE not null,
  end_time  TIMESTAMP with TIME ZONE not null,
  name VARCHAR not null,
  description text not null,
  CHECK(char_length(DESCRIPTION) <= 1000)
);
create index ID_INDEX on events using HASH(id);
