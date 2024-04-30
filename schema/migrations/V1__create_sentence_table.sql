CREATE TABLE sentence (
    id BIGSERIAL PRIMARY KEY,
    version SMALLINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    text TEXT NOT NULL
);

CREATE INDEX idx_sentence_updated_at ON sentence (updated_at);

CREATE TABLE sentence_history (
    id BIGINT,
    version SMALLINT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    text TEXT NOT NULL,
    PRIMARY KEY(id, version)
);

CREATE OR REPLACE FUNCTION update_sentence() RETURNS TRIGGER AS $$
BEGIN
    NEW.version = OLD.version + 1;
    NEW.updated_at = NOW();

    INSERT INTO sentence_history
        (id, version, created_at, text)
    VALUES
        (OLD.id, OLD.version, NEW.updated_at, OLD.text);

    RETURN NEW;
END;
$$ LANGUAGE 'plpgsql';

CREATE TRIGGER update_sentence
BEFORE UPDATE ON sentence FOR EACH ROW
EXECUTE PROCEDURE update_sentence();

