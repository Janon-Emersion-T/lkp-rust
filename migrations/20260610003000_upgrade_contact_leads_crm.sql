ALTER TABLE contact_messages
    ADD COLUMN IF NOT EXISTS internal_note TEXT,
    ADD COLUMN IF NOT EXISTS next_follow_up_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS contacted_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS qualified_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS converted_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS archived_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS spam_at TIMESTAMPTZ,
    ADD COLUMN IF NOT EXISTS lost_reason TEXT,
    ADD COLUMN IF NOT EXISTS assigned_to TEXT,
    ADD COLUMN IF NOT EXISTS client_ip TEXT,
    ADD COLUMN IF NOT EXISTS user_agent TEXT;

CREATE INDEX IF NOT EXISTS idx_contact_messages_lead_score
    ON contact_messages(lead_score DESC);

CREATE INDEX IF NOT EXISTS idx_contact_messages_next_follow_up_at
    ON contact_messages(next_follow_up_at);

CREATE INDEX IF NOT EXISTS idx_contact_messages_service_interest
    ON contact_messages(service_interest);

CREATE INDEX IF NOT EXISTS idx_contact_messages_archived_at
    ON contact_messages(archived_at);

CREATE INDEX IF NOT EXISTS idx_contact_messages_spam_at
    ON contact_messages(spam_at);