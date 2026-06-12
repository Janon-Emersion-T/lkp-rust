CREATE INDEX IF NOT EXISTS idx_contact_messages_status
ON contact_messages (status);

CREATE INDEX IF NOT EXISTS idx_contact_messages_priority
ON contact_messages (priority);

CREATE INDEX IF NOT EXISTS idx_contact_messages_service_interest
ON contact_messages (service_interest);

CREATE INDEX IF NOT EXISTS idx_contact_messages_created_at
ON contact_messages (created_at DESC);

CREATE INDEX IF NOT EXISTS idx_contact_messages_lead_score
ON contact_messages (lead_score DESC);

CREATE INDEX IF NOT EXISTS idx_contact_messages_status_priority_score
ON contact_messages (status, priority, lead_score DESC, created_at DESC);