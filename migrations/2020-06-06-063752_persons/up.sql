CREATE TYPE animal_type AS ENUM (
    'Cat',
    'Fish'
    );

CREATE TYPE pet AS
(
    name        text,
    animal_type animal_type
);

CREATE TABLE persons
(
    id   serial not null
        constraint person_pk
            primary key,
    name varchar not null,
    age  int not null,
    pets pet[] not null
);-- Your SQL goes here