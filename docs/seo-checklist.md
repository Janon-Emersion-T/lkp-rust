# LKProfessionals SEO, AEO, GEO, LLMO, and SXO Checklist

## Foundation

- Use one clear `H1` per page and keep section headings in strict `H2`/`H3` order.
- Keep canonical URLs absolute and aligned with public routes.
- Publish unique title tags and meta descriptions for every public page.
- Keep primary pages indexable and block dashboard/admin routes from crawl surfaces.
- Maintain `robots.txt`, `llms.txt`, XML sitemap, and HTML sitemap together.

## Technical SEO

- Prefer semantic HTML landmarks: `header`, `nav`, `main`, `section`, `article`, `aside`, `footer`.
- Add `hreflang` for `en-LK`, `en`, and `x-default` on canonical pages.
- Keep CSS payload small and static-first. Avoid runtime style generation.
- Preload the main stylesheet and only preconnect to hosts actually used above the fold.
- Use `loading=\"lazy\"` and `decoding=\"async\"` on non-critical images.
- Avoid layout shift by ensuring predictable image dimensions or stable containers.
- Use lightweight Alpine only for interactions that materially improve UX.

## Structured Data

- Site-wide graph should include `Organization`, `WebSite`, and `LocalBusiness`.
- Service detail pages should expose `Service` schema.
- Insight detail pages should expose `Article` schema with author, published date, updated date, and canonical URL.
- Case study detail pages should expose `CreativeWork` or `Article` style schema with project outcome detail.
- FAQ-heavy pages should expose `FAQPage` only when visible Q&A content exists on the page.
- Breadcrumb trails should output `BreadcrumbList` schema on detail pages.

## On-Page SEO

- Put a direct answer block near the top of commercial and informational pages.
- Use Sri Lanka and Jaffna modifiers naturally where commercially relevant.
- Add contextual internal links from services to case studies, insights, industries, and contact points.
- Keep paragraphs tight, factual, and easy to quote.
- Use numbered steps, tables, bullet lists, and definition-style sections where clarity improves retrieval.

## AEO / GEO / LLMO

- Answer obvious buyer questions directly before expanding into supporting context.
- Add visible author identity and dates on insight pages.
- Prefer original operational detail over abstract marketing language.
- Include process descriptions, before/after states, and measurable outcomes when available.
- Use summary sections that LLMs can quote without needing to infer missing context.
- Keep business claims grounded in evidence published on the page.

## Local SEO

- Keep NAP consistent in header, footer, contact page, schema, and discovery files.
- Keep Jaffna and Sri Lanka mentions commercially relevant rather than stuffed.
- Use contact, service area, and industry pages to reinforce real market coverage.
- Add exact `geo` coordinates only when they are verified from a trusted business source.

## SXO / CRO / Accessibility

- Keep a visible skip link and keyboard-focus states site-wide.
- Ensure forms have labels, validation hints, and clear success/error messaging.
- Keep primary CTAs visible above the fold and repeated after proof sections.
- Add breadcrumb navigation on deeper pages to reduce pogo-sticking.
- Use trust sections with specific proof instead of generic claims.
- Maintain accessible color contrast and non-color state cues.

## Content Operations

- Expand case studies with challenge, solution, measurable results, technology, and testimonial fields.
- Add FAQ coverage to service and contact pages based on real buyer questions.
- Keep insights internally linked to relevant service and case study pages.
- Refresh stale metadata when page positioning changes.

## Build-Time Optimization

- Generate optimized WebP/AVIF derivatives for local hero assets.
- Prefer self-hosted, compressed assets for logos and recurring UI imagery.
- Audit third-party CSS and JS regularly; remove anything not essential.
- Keep Tailwind input concise and avoid unused bespoke utility duplication.
- Add a pre-deploy check that runs `cargo fmt`, `cargo check`, and asset builds.
