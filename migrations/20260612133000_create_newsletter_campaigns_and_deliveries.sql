ALTER TABLE newsletter_subscribers
ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW();

CREATE TABLE IF NOT EXISTS newsletter_campaigns (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    subject TEXT NOT NULL,
    preview_text TEXT,
    content_html TEXT NOT NULL,
    cta_label TEXT,
    cta_url TEXT,
    source_type TEXT NOT NULL DEFAULT 'manual',
    source_id UUID,
    status TEXT NOT NULL DEFAULT 'queued',
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_newsletter_campaigns_source
ON newsletter_campaigns (source_type, source_id)
WHERE source_id IS NOT NULL;

CREATE TABLE IF NOT EXISTS newsletter_deliveries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    campaign_id UUID NOT NULL REFERENCES newsletter_campaigns(id) ON DELETE CASCADE,
    subscriber_id UUID REFERENCES newsletter_subscribers(id) ON DELETE SET NULL,
    email TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    last_attempt_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (campaign_id, email)
);

CREATE INDEX IF NOT EXISTS idx_newsletter_deliveries_pending
ON newsletter_deliveries (status, created_at);
