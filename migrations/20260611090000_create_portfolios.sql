CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE IF NOT EXISTS portfolios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    client_name TEXT,
    industry TEXT,
    service_category TEXT,
    excerpt TEXT NOT NULL,
    overview TEXT NOT NULL,
    challenge TEXT,
    solution TEXT,
    results TEXT,
    impact_metrics TEXT,
    technologies TEXT,
    cover_image_url TEXT,
    live_url TEXT,
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    meta_title TEXT,
    meta_description TEXT,
    canonical_url TEXT,
    og_image_url TEXT,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_portfolios_published
    ON portfolios(published, featured, sort_order, published_at DESC, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_portfolios_slug
    ON portfolios(slug);

CREATE INDEX IF NOT EXISTS idx_portfolios_industry
    ON portfolios(industry);
