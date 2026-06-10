CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS contact_messages (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    company TEXT,
    service_interest TEXT,
    budget_range TEXT,
    project_timeline TEXT,
    subject TEXT NOT NULL,
    message TEXT NOT NULL,
    source TEXT NOT NULL DEFAULT 'contact_page',
    status TEXT NOT NULL DEFAULT 'new',
    priority TEXT NOT NULL DEFAULT 'normal',
    lead_score INT NOT NULL DEFAULT 0,
    admin_reply TEXT,
    replied_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE contact_messages
    ADD COLUMN IF NOT EXISTS service_interest TEXT,
    ADD COLUMN IF NOT EXISTS budget_range TEXT,
    ADD COLUMN IF NOT EXISTS project_timeline TEXT,
    ADD COLUMN IF NOT EXISTS source TEXT NOT NULL DEFAULT 'contact_page',
    ADD COLUMN IF NOT EXISTS priority TEXT NOT NULL DEFAULT 'normal',
    ADD COLUMN IF NOT EXISTS lead_score INT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE INDEX IF NOT EXISTS idx_contact_messages_status
    ON contact_messages(status);

CREATE INDEX IF NOT EXISTS idx_contact_messages_created_at
    ON contact_messages(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_contact_messages_priority
    ON contact_messages(priority);

CREATE INDEX IF NOT EXISTS idx_contact_messages_email
    ON contact_messages(email);