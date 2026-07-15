-- Sequence backing Zetra ID numbers, starting at ZT100000001
CREATE SEQUENCE IF NOT EXISTS zetra_id_seq START 100000001;

ALTER TABLE users
    ADD COLUMN IF NOT EXISTS zetra_id TEXT,
    ADD COLUMN IF NOT EXISTS zetramail TEXT,
    ADD COLUMN IF NOT EXISTS phone TEXT;

-- Backfill any existing users
UPDATE users
SET zetra_id = 'ZT' || nextval('zetra_id_seq')
WHERE zetra_id IS NULL;

UPDATE users
SET zetramail = lower(username) || '@ztmail.zt'
WHERE zetramail IS NULL;

-- Lock in NOT NULL now that every row is backfilled
ALTER TABLE users
    ALTER COLUMN zetra_id SET NOT NULL,
    ALTER COLUMN zetramail SET NOT NULL;

-- Auto-generate zetra_id for all future inserts (no app code needed)
ALTER TABLE users
    ALTER COLUMN zetra_id SET DEFAULT ('ZT' || nextval('zetra_id_seq'));

-- Uniqueness
ALTER TABLE users
    ADD CONSTRAINT users_zetra_id_key UNIQUE (zetra_id),
    ADD CONSTRAINT users_zetramail_key UNIQUE (zetramail),
    ADD CONSTRAINT users_phone_key UNIQUE (phone);

CREATE INDEX IF NOT EXISTS users_zetra_id_idx ON users (zetra_id);
CREATE INDEX IF NOT EXISTS users_zetramail_idx ON users (zetramail);
CREATE INDEX IF NOT EXISTS users_phone_idx ON users (phone) WHERE phone IS NOT NULL;
