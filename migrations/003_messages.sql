CREATE TABLE IF NOT EXISTS service_apps (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,          -- e.g. 'nai', 'nigergram', 'ztc'
    api_key_hash TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    from_app TEXT NOT NULL,             -- service_apps.name, denormalized for easy display
    kind TEXT NOT NULL DEFAULT 'verification_code',
    subject TEXT NOT NULL,
    body TEXT NOT NULL,
    code TEXT,                          -- the actual OTP, if kind = 'verification_code'
    read_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS messages_user_idx ON messages (user_id, created_at DESC);
CREATE INDEX IF NOT EXISTS messages_user_unread_idx ON messages (user_id) WHERE read_at IS NULL;
