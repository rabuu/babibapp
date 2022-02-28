CREATE TABLE students (
	id SERIAL PRIMARY KEY,
	email TEXT UNIQUE NOT NULL,
	first_name VARCHAR(20) NOT NULL,
	last_name VARCHAR(20) NOT NULL,
	password_hash TEXT NOT NULL,
	is_admin BOOLEAN DEFAULT FALSE NOT NULL,
	UNIQUE(first_name, last_name)
);
