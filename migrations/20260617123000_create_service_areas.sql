CREATE TABLE IF NOT EXISTS service_areas (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    area_name TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    area_type TEXT NOT NULL DEFAULT 'city',
    country TEXT NOT NULL,
    market_region TEXT NOT NULL,
    short_description TEXT NOT NULL,
    overview TEXT NOT NULL,
    buyer_profile TEXT,
    delivery_focus TEXT,
    timezone_note TEXT,
    nearby_markets TEXT,
    hero_image_url TEXT,
    gallery_image_url_2 TEXT,
    gallery_image_url_3 TEXT,
    featured BOOLEAN NOT NULL DEFAULT FALSE,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    sort_order INTEGER NOT NULL DEFAULT 0,
    meta_title TEXT,
    meta_description TEXT,
    canonical_url TEXT,
    og_image_url TEXT,
    published_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT service_areas_area_type_check CHECK (area_type IN ('city', 'state', 'country'))
);

CREATE INDEX IF NOT EXISTS idx_service_areas_public_listing
    ON service_areas (published, featured, market_region, sort_order, published_at DESC, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_service_areas_slug
    ON service_areas (slug);

CREATE INDEX IF NOT EXISTS idx_service_areas_region
    ON service_areas (market_region, country);
