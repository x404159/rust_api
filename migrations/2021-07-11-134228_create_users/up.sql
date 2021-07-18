-- Your SQL goes here
CREATE TABLE  users (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR (100) NOT NULL, 
    email VARCHAR (100) UNIQUE NOT NULL,
    password VARCHAR (122) NOT NULL,
    clearance BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT now()
)