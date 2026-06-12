use chrono::{Duration, Utc};
use sqlx::PgPool;

type SeedResult<T> = Result<T, sqlx::Error>;

struct SeedCareer<'a> {
    title: &'a str,
    slug: &'a str,
    team: &'a str,
    location: &'a str,
    workplace_mode: &'a str,
    employment_type: &'a str,
    experience_level: &'a str,
    salary_range: &'a str,
    summary: &'a str,
    description: &'a str,
    responsibilities: &'a str,
    requirements: &'a str,
    benefits: &'a str,
    cover_image_url: &'a str,
    featured: bool,
    sort_order: i32,
}

const DEFAULT_CAREERS: &[SeedCareer] = &[
    SeedCareer {
        title: "Senior Laravel Developer",
        slug: "senior-laravel-developer",
        team: "Engineering",
        location: "Jaffna, Sri Lanka",
        workplace_mode: "Hybrid",
        employment_type: "Full-time",
        experience_level: "Senior",
        salary_range: "Competitive based on experience",
        summary: "Lead backend delivery for business systems, custom platforms, and web products built for serious operational use.",
        description: "LKProfessionals is hiring a Senior Laravel Developer to take ownership of API design, business logic architecture, and production-quality delivery across client platforms.\n\nThis role fits someone who can move beyond tickets, shape implementation decisions, guide code quality, and keep delivery grounded in business outcomes.",
        responsibilities: "- Design and maintain Laravel applications for production use\n- Review architecture choices and unblock implementation teams\n- Build stable APIs, admin tools, and workflow-heavy modules\n- Collaborate with frontend, SEO, and operations stakeholders\n- Improve delivery quality through clean structure and practical standards",
        requirements: "- Strong professional Laravel and PHP experience\n- Comfortable with MySQL, queueing, authentication, and integrations\n- Able to reason about maintainability and delivery tradeoffs\n- Clear written communication and ownership mindset\n- Experience shipping real client-facing or operations-facing systems",
        benefits: "- Meaningful client work with real operational impact\n- Exposure to modern delivery across web, software, and SEO systems\n- Flexible hybrid collaboration model\n- Growth path into technical leadership responsibilities",
        cover_image_url: "https://images.unsplash.com/photo-1516321318423-f06f85e504b3?auto=format&fit=crop&w=1400&q=80",
        featured: true,
        sort_order: 1,
    },
    SeedCareer {
        title: "Frontend Developer",
        slug: "frontend-developer",
        team: "Engineering",
        location: "Jaffna, Sri Lanka",
        workplace_mode: "Hybrid",
        employment_type: "Full-time",
        experience_level: "Mid-level",
        salary_range: "Competitive based on capability",
        summary: "Build high-performing interfaces for service pages, business dashboards, and lead-focused marketing sites.",
        description: "We are looking for a Frontend Developer who cares about detail, performance, accessibility, and conversion quality.\n\nYou will work across public websites and internal systems, translating structured content and business goals into polished interfaces that feel deliberate rather than generic.",
        responsibilities: "- Build responsive frontend experiences with clean structure\n- Translate strategy and design direction into production-ready UI\n- Improve performance, accessibility, and conversion flow\n- Work closely with backend and SEO collaborators\n- Maintain consistency across reusable interface patterns",
        requirements: "- Strong HTML, CSS, JavaScript, and modern frontend workflow knowledge\n- Experience with responsive implementation and accessibility\n- Good taste in layout, hierarchy, and interaction detail\n- Able to work from briefs and improve weak requirements proactively",
        benefits: "- Wide range of project types instead of repetitive product maintenance\n- Real influence on visible client outcomes\n- Space to improve systems, not just implement screens\n- Supportive path into broader product and UX ownership",
        cover_image_url: "https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=1400&q=80",
        featured: true,
        sort_order: 2,
    },
    SeedCareer {
        title: "SEO Content Strategist",
        slug: "seo-content-strategist",
        team: "Growth",
        location: "Jaffna, Sri Lanka",
        workplace_mode: "Remote-friendly",
        employment_type: "Full-time",
        experience_level: "Mid-level",
        salary_range: "Open to strong candidates",
        summary: "Shape service pages, insight programs, and topical authority strategies that help LKProfessionals and client brands rank and convert.",
        description: "This role sits between strategy, content operations, and search performance.\n\nYou will help define content structures, identify intent gaps, improve on-page quality, and contribute to an answer-engine-ready publishing workflow.",
        responsibilities: "- Plan and refine SEO content strategy across services and insights\n- Write or structure content briefs with strong search intent alignment\n- Improve metadata, internal linking, and content quality signals\n- Collaborate with developers on structured data and technical SEO execution\n- Track what content themes produce trust and commercial traction",
        requirements: "- Solid understanding of SEO fundamentals and content strategy\n- Strong writing and editorial judgment\n- Comfortable working with keyword themes, search intent, and page architecture\n- Able to think commercially, not just informationally",
        benefits: "- Direct influence on brand authority and inbound growth\n- Work at the intersection of SEO, AEO, engineering, and conversion\n- Opportunity to shape a growing content engine from the inside",
        cover_image_url: "https://images.unsplash.com/photo-1460925895917-afdab827c52f?auto=format&fit=crop&w=1400&q=80",
        featured: false,
        sort_order: 3,
    },
];

pub async fn seed_default_careers(pool: &PgPool) -> SeedResult<()> {
    for (index, career) in DEFAULT_CAREERS.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO careers
            (
                title, slug, team, location, workplace_mode, employment_type, experience_level,
                salary_range, summary, description, responsibilities, requirements, benefits,
                application_email, cover_image_url, featured, published, sort_order, meta_title,
                meta_description, canonical_url, og_image_url, published_at
            )
            VALUES
            (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13,
                'careers@lkprofessionals.com', $14, $15, TRUE, $16, $17, $18, $19, $20, $21
            )
            ON CONFLICT (slug) DO UPDATE SET
                title = EXCLUDED.title,
                team = EXCLUDED.team,
                location = EXCLUDED.location,
                workplace_mode = EXCLUDED.workplace_mode,
                employment_type = EXCLUDED.employment_type,
                experience_level = EXCLUDED.experience_level,
                salary_range = EXCLUDED.salary_range,
                summary = EXCLUDED.summary,
                description = EXCLUDED.description,
                responsibilities = EXCLUDED.responsibilities,
                requirements = EXCLUDED.requirements,
                benefits = EXCLUDED.benefits,
                application_email = EXCLUDED.application_email,
                cover_image_url = EXCLUDED.cover_image_url,
                featured = EXCLUDED.featured,
                published = EXCLUDED.published,
                sort_order = EXCLUDED.sort_order,
                meta_title = EXCLUDED.meta_title,
                meta_description = EXCLUDED.meta_description,
                canonical_url = EXCLUDED.canonical_url,
                og_image_url = EXCLUDED.og_image_url,
                published_at = EXCLUDED.published_at,
                updated_at = NOW()
            "#,
        )
        .bind(career.title)
        .bind(career.slug)
        .bind(career.team)
        .bind(career.location)
        .bind(career.workplace_mode)
        .bind(career.employment_type)
        .bind(career.experience_level)
        .bind(career.salary_range)
        .bind(career.summary)
        .bind(career.description)
        .bind(career.responsibilities)
        .bind(career.requirements)
        .bind(career.benefits)
        .bind(career.cover_image_url)
        .bind(career.featured)
        .bind(career.sort_order)
        .bind(format!("{} | Careers at LKProfessionals", career.title))
        .bind(career.summary)
        .bind(format!(
            "https://lkprofessionals.com/careers/{}",
            career.slug
        ))
        .bind(career.cover_image_url)
        .bind(Utc::now() - Duration::days(index as i64))
        .execute(pool)
        .await?;
    }

    Ok(())
}
