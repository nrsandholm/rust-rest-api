CREATE TABLE applications_applicants (
	id SERIAL PRIMARY KEY,
	application_id SERIAL REFERENCES applications(id),
	applicant_id SERIAL REFERENCES applicants(id),
	UNIQUE(application_id, applicant_id)
)