ALTER TABLE portfolios
    ADD COLUMN IF NOT EXISTS testimonial_quote TEXT,
    ADD COLUMN IF NOT EXISTS testimonial_author TEXT,
    ADD COLUMN IF NOT EXISTS testimonial_author_role TEXT;
