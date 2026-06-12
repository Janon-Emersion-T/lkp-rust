CREATE TABLE IF NOT EXISTS careers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    team TEXT,
    location TEXT NOT NULL DEFAULT 'Jaffna, Sri Lanka',
    workplace_mode TEXT NOT NULL DEFAULT 'Hybrid',
    employment_type TEXT NOT NULL DEFAULT 'Full-time',
    experience_level TEXT NOT NULL DEFAULT 'Mid-level',
    salary_range TEXT,
    summary TEXT NOT NULL,
    description TEXT NOT NULL,
    responsibilities TEXT,
    requirements TEXT,
    benefits TEXT,
    application_email TEXT,
    apply_url TEXT,
    cover_image_url TEXT,
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

CREATE INDEX IF NOT EXISTS idx_careers_public_listing
ON careers (published, featured, sort_order, published_at DESC, created_at DESC);

CREATE TABLE IF NOT EXISTS career_applications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    career_id UUID REFERENCES careers(id) ON DELETE SET NULL,
    role_title_snapshot TEXT NOT NULL,
    full_name TEXT NOT NULL,
    email TEXT NOT NULL,
    phone TEXT,
    location TEXT,
    linkedin_url TEXT,
    portfolio_url TEXT,
    resume_url TEXT,
    cover_letter TEXT NOT NULL,
    experience_summary TEXT,
    availability TEXT,
    expected_salary TEXT,
    source TEXT,
    status TEXT NOT NULL DEFAULT 'new',
    internal_notes TEXT,
    reviewed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_career_applications_status
ON career_applications (status, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_career_applications_career
ON career_applications (career_id, created_at DESC);
