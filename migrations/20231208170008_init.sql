
create table planes (
  id uuid primary key,
  name varchar(255) not null,
  capacity int not null
);


create table flights (
  id uuid primary key,
  plane_id uuid not null references planes(id),
  origin varchar(255) not null,
  destination varchar(255) not null,
  departure_time timestamp not null,
  status varchar(255) not null
);