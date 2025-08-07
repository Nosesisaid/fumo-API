-- Your SQL goes here
CREATE TABLE fumos (
    id BIGSERIAL PRIMARY KEY,
    caption VARCHAR NOT NULL,
    img VARCHAR NOT NULL,
    public BOOLEAN NOT NULL DEFAULT FALSE,
    involved TEXT [] NOT NULL,
    attribution TEXT NOT NULL,
    submission_time TIMESTAMP NOT NULL DEFAULT NOW(),
    submitter TEXT NOT NULL
);
