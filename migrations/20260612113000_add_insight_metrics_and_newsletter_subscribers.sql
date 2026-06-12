ALTER TABLE insights
ADD COLUMN IF NOT EXISTS view_count INTEGER NOT NULL DEFAULT 0;

WITH ranked_insights AS (
    SELECT
        id,
        ROW_NUMBER() OVER (ORDER BY featured DESC, published_at DESC NULLS LAST, created_at DESC) AS position,
        featured
    FROM insights
)
UPDATE insights
SET view_count = CASE
    WHEN ranked_insights.featured THEN 180 + (ranked_insights.position * 31)
    ELSE 40 + (ranked_insights.position * 9)
END
FROM ranked_insights
WHERE insights.id = ranked_insights.id
  AND insights.view_count = 0;

CREATE TABLE IF NOT EXISTS newsletter_subscribers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    source TEXT,
    subscribed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
