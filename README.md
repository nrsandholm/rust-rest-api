# RUST REST API

Rocket as web framework (https://rocket.rs/)

Diesel as ORM (http://diesel.rs/)

## Run

1. docker up
2. cargo run

## Resources

Method | URL | Action
--- | --- | ---
POST | /api/applications | create_application
GET | /api/applications | read_applications
GET | /api/applications/{id} | read_application
GET | /api/applications/{id}?relations=true | read_application_with_relations
PUT | /api/applications/{id}/name | update_application_name
DELETE | /api/applications/{id} | delete_application
POST | /api/applications/{id}/files | create_file
GET | /api/applications/{id}/files | read_files
DELETE | /api/applications/{id}/files | delete_file
POST | /api/applications/{id}/applicants | create_applicant
GET | /api/applicants | read_applicants
GET | /api/applicants/{id} | read_applicant
DELETE | /api/applicants/{id} | delete_applicant

## Models

See src/models.rs
