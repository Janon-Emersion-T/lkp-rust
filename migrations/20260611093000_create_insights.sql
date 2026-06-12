CREATE TABLE IF NOT EXISTS insights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    excerpt TEXT NOT NULL,
    content TEXT NOT NULL,
    author TEXT NOT NULL DEFAULT 'LKProfessionals Team',
    category TEXT,
    cover_image_url TEXT,
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    reading_time_minutes INTEGER NOT NULL DEFAULT 5,
    meta_title TEXT,
    meta_description TEXT,
    canonical_url TEXT,
    og_image_url TEXT,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_insights_published
    ON insights(published, featured, published_at DESC, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_insights_slug
    ON insights(slug);

CREATE INDEX IF NOT EXISTS idx_insights_category
    ON insights(category);
