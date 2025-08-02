-- Your SQL goes here
CREATE TABLE fumos (
    id BIGSERIAL PRIMARY KEY,
    caption VARCHAR NOT NULL,
    img VARCHAR NOT NULL,
    public BOOLEAN NOT NULL DEFAULT FALSE,
    involved TEXT [],
    attribution TEXT NOT NULL,
    submission_time TIMESTAMP NOT NULL DEFAULT NOW(),
    submitter VARCHAR(32) NOT NULL
);
