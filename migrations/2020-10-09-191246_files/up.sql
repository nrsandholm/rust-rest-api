CREATE TABLE files (
	id SERIAL PRIMARY KEY,
	application_id INT NOT NULL REFERENCES applications(id),
	uri VARCHAR NOT NULL
)
