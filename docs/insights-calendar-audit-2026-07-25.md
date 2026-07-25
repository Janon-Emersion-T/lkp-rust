# Insights Calendar Audit

Date audited: 2026-07-25

## Architecture findings

- Insight records are stored in `src/seeds/data/insights.json` and seeded through `src/seeds/insights.rs`.
- The persisted model is `InsightRecord` in `src/models/insight.rs`.
- Current insight fields: `title`, `slug`, `excerpt`, `content`, `author`, `category`, `cover_image_url`, `featured`, `published`, `view_count`, `reading_time_minutes`, `meta_title`, `meta_description`, `canonical_url`, `og_image_url`, `published_at`, `created_at`, `updated_at`.
- There is no implemented per-insight tag relationship in the active model or seed dataset. Dashboard routes for tags and categories exist as placeholders, but content records currently store only a single category string.
- Public insight routes are `/insights`, `/insights/category/{slug}`, and `/insights/{slug}`.
- Article structured data is emitted on `templates/pages/insights/single.html` using `Article` and `BreadcrumbList` schema.
- Sitemap integration is dynamic through `src/handlers/discovery.rs`, which pulls published insight slugs from the `insights` table.
- Canonical, Open Graph, and Twitter metadata are already supported at the template level.
- Existing cover images use absolute `https://lkprofessionals.com/serve/insights/featured/...` URLs.
- Slugs are clean keyword-first slugs with no date component.
- Authors already in use for the active 2026 programme are `LKProfessionals Team`, `LKProfessionals Editorial Team`, `LKProfessionals Strategy Desk`, `LKProfessionals Growth Team`, `LKProfessionals Web Team`, and `Janon Emersion T`.

## Publication-pattern findings

- Total published insights in the active 2026 programme: 214
- Earliest relevant publication date: 2026-01-01
- Latest existing publication date: 2026-06-12
- Established publishing frequency: Daily
- Notes on outliers:
  - There are isolated legacy entries dated 2012-10-12 and 2013-10-12 through 2014-01-19.
  - Those dates do not match the current uninterrupted 2026 publishing programme and were excluded from the active cadence analysis.
  - Several 2026 dates contain multiple insights, but the baseline schedule still shows at least one published insight every day from 2026-01-01 through 2026-06-12.

## Missing publication dates to fill

- 2026-06-13
- 2026-06-14
- 2026-06-15
- 2026-06-16
- 2026-06-17
- 2026-06-18
- 2026-06-19
- 2026-06-20
- 2026-06-21
- 2026-06-22
- 2026-06-23
- 2026-06-24
- 2026-06-25
- 2026-06-26
- 2026-06-27
- 2026-06-28
- 2026-06-29
- 2026-06-30
- 2026-07-01
- 2026-07-02
- 2026-07-03
- 2026-07-04
- 2026-07-05
- 2026-07-06
- 2026-07-07
- 2026-07-08
- 2026-07-09
- 2026-07-10
- 2026-07-11
- 2026-07-12
- 2026-07-13
- 2026-07-14
- 2026-07-15
- 2026-07-16
- 2026-07-17
- 2026-07-18
- 2026-07-19
- 2026-07-20
- 2026-07-21
- 2026-07-22
- 2026-07-23
- 2026-07-24
- 2026-07-25

Total missing publication dates: 43
