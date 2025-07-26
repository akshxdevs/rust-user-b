-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- Create Enum
CREATE TYPE user_status AS ENUM ('Active', 'ideal', 'dead');

-- Create Tables
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    createdAt TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
