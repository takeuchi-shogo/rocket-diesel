-- Your SQL goes here
CREATE TABLE users
(
	id SERIAL PRIMARY KEY,
	display_name varchar(255) NOT NULL,
	password varchar(255) NOT NULL
);