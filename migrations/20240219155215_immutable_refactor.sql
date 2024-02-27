alter table planes drop column name;
alter table planes add column deleted boolean not null default false;

alter table airports add column deleted boolean not null default false;

alter table flights drop column status;
alter table flights drop column departure_gate;
alter table flights drop column arrival_gate;

create table flight_cancellations (
    flight_id uuid not null references flights(id),
    timestamp timestamp with time zone not null default now(),
    reason varchar
);

create table flight_delays (
    flight_id uuid not null references flights(id),
    timestamp timestamp with time zone not null default now(),
    departure_time timestamp with time zone not null,
    arrival_time timestamp with time zone not null
);

create table flight_departure_gates (
    flight_id uuid not null references flights(id),
    timestamp timestamp with time zone not null default now(),
    gate varchar not null
);

create table flight_arrival_gates (
    flight_id uuid not null references flights(id),
    timestamp timestamp with time zone not null default now(),
    gate varchar not null
);