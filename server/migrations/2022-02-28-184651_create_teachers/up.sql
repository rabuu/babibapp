CREATE TABLE teachers (
	id SERIAL PRIMARY KEY,
	name VARCHAR(30) NOT NULL,
	prefix VARCHAR(10) NOT NULL,
	UNIQUE(name, prefix)
);
