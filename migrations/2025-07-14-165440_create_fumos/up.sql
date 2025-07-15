
CREATE TABLE fumos (
    id BIGSERIAL PRIMARY KEY,
    caption VARCHAR NOT NULL,
    img VARCHAR NOT NULL,
    public BOOLEAN DEFAULT FALSE,
    involved TEXT[] NOT NULL,
    attribution TEXT
);


