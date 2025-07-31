-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE user_status AS ENUM ('Active', 'ideal', 'dead');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    createdat TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
