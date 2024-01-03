
create table planes (
  id uuid primary key,
  name varchar not null,
  model varchar not null,
  cabin_capacity int not null,
  cargo_capacity_kg int not null
);


create table airports (
  id uuid primary key,
  icao varchar(4) not null,
  iata varchar(3) not null,
  name varchar not null,
  country varchar not null,
  city varchar not null
);


create table flights (
  id uuid primary key,
  plane_id uuid not null references planes(id),
  origin_id uuid not null references airports(id),
  destination_id uuid not null references airports(id),
  departure_time timestamp with time zone not null,
  arrival_time timestamp with time zone not null,
  departure_gate varchar not null,
  arrival_gate varchar not null,
  status varchar not null
);