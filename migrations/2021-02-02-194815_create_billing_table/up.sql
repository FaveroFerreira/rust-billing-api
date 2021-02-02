-- Your SQL goes here
create table billing(
    id serial primary key,
    title varchar(255) not null,
    amount numeric(8,2) not null,
    paid boolean not null default false
);