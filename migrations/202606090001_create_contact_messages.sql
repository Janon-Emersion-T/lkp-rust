CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS contact_messages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

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
    lead_score INTEGER NOT NULL DEFAULT 0,

    admin_reply TEXT,
    internal_note TEXT,
    assigned_to TEXT,
    lost_reason TEXT,

    next_follow_up_at TIMESTAMPTZ,

    contacted_at TIMESTAMPTZ,
    qualified_at TIMESTAMPTZ,
    converted_at TIMESTAMPTZ,
    archived_at TIMESTAMPTZ,
    spam_at TIMESTAMPTZ,
    replied_at TIMESTAMPTZ,

    client_ip TEXT,
    user_agent TEXT,

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT contact_messages_status_check CHECK (
        status IN ('new', 'contacted', 'qualified', 'converted', 'archived', 'spam')
    ),

    CONSTRAINT contact_messages_priority_check CHECK (
        priority IN ('normal', 'medium', 'high')
    ),

    CONSTRAINT contact_messages_lead_score_check CHECK (
        lead_score >= 0 AND lead_score <= 100
    )
);

CREATE INDEX IF NOT EXISTS idx_contact_messages_status
    ON contact_messages(status);

CREATE INDEX IF NOT EXISTS idx_contact_messages_priority
    ON contact_messages(priority);

CREATE INDEX IF NOT EXISTS idx_contact_messages_service_interest
    ON contact_messages(service_interest);

CREATE INDEX IF NOT EXISTS idx_contact_messages_created_at
    ON contact_messages(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_contact_messages_next_follow_up_at
    ON contact_messages(next_follow_up_at);

CREATE INDEX IF NOT EXISTS idx_contact_messages_search
    ON contact_messages
    USING gin (
        to_tsvector(
            'english',
            coalesce(name, '') || ' ' ||
            coalesce(email, '') || ' ' ||
            coalesce(phone, '') || ' ' ||
            coalesce(company, '') || ' ' ||
            coalesce(subject, '') || ' ' ||
            coalesce(message, '')
        )
    );