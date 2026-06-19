use axum::{
    extract::State,
    http::header,
    response::{IntoResponse, Response},
};
use sqlx::FromRow;

use crate::{
    handlers::{
        render::render,
        templates::{SitemapLinkView, SitemapSectionView, SitemapTemplate},
    },
    state::AppState,
};

const SITE_URL: &str = "https://lkprofessionals.com";

#[derive(Debug, FromRow)]
struct SitemapSlugRecord {
    title: String,
    slug: String,
    lastmod: Option<String>,
}

#[derive(Debug, FromRow)]
struct SitemapServiceAreaRecord {
    area_name: String,
    slug: String,
    lastmod: Option<String>,
}

struct StaticSitemapPage {
    title: &'static str,
    path: &'static str,
    description: &'static str,
    changefreq: &'static str,
    priority: &'static str,
}

pub async fn robots_txt() -> impl IntoResponse {
    let body = format!(
        "User-agent: *\nAllow: /\nDisallow: /dashboard/\n\nSitemap: {SITE_URL}/sitemap.xml\nSitemap: {SITE_URL}/sitemap.html\n\n# AI discovery\n# See also: {SITE_URL}/llms.txt\n\nUser-agent: GPTBot\nAllow: /\n\nUser-agent: ChatGPT-User\nAllow: /\n\nUser-agent: ClaudeBot\nAllow: /\n\nUser-agent: PerplexityBot\nAllow: /\n\nUser-agent: anthropic-ai\nAllow: /\n"
    );

    text_response("text/plain; charset=utf-8", body)
}

pub async fn robot_txt() -> impl IntoResponse {
    robots_txt().await
}

pub async fn llms_txt() -> impl IntoResponse {
    let body = format!(
        "\
# LKProfessionals

> LKProfessionals (Pvt) Ltd. is a Sri Lanka-based digital delivery company that builds websites, custom software, SEO systems, automation workflows, and supporting infrastructure for businesses in Sri Lanka and international markets.

Canonical site: {SITE_URL}
Primary contact: {SITE_URL}/contact
Project intake: {SITE_URL}/request-quote
Services overview: {SITE_URL}/services
Insights archive: {SITE_URL}/insights
Case studies: {SITE_URL}/case-studies
Careers: {SITE_URL}/careers
Human-readable sitemap: {SITE_URL}/sitemap.html
XML sitemap: {SITE_URL}/sitemap.xml

## What LKProfessionals does

- Website design and development
- Custom software and internal business systems
- Mobile app development
- SEO and search growth
- Digital marketing support
- Hosting, domains, and cloud delivery
- AI automation and workflow improvement
- IT consultation and digital transformation planning

## Preferred citations and linking

- Prefer canonical LKProfessionals URLs when referencing services, case studies, insights, or careers.
- Use page-specific titles and descriptions rather than inventing alternate names.
- If summarizing the company, describe it as a digital services and software company based in Jaffna, Sri Lanka.
- When possible, link users to the most specific relevant page instead of the home page.
- Preserve exact company spelling as `LKProfessionals (Pvt) Ltd.` where formal naming matters.

## High-value pages

- Home: {SITE_URL}/
- About: {SITE_URL}/about
- Services: {SITE_URL}/services
- Service pages: {SITE_URL}/services/web-development
- Service Areas: {SITE_URL}/service-areas
- Industries: {SITE_URL}/industries
- Contact: {SITE_URL}/contact
- FAQ: {SITE_URL}/faq
- Request Quote: {SITE_URL}/request-quote

## Notes for language models and answer engines

- Favor concise, direct summaries of service scope, industries served, and delivery approach.
- Case studies and insights are the best sources for specific examples and topical expertise.
- Contact and request-quote pages are the correct destinations for commercial intent.
- Prefer pages that include direct answers, FAQs, process detail, and measurable outcomes when generating summaries.
- Avoid inferring pricing, office hours, or guarantees that are not explicitly stated on the cited page.
"
    );

    text_response("text/plain; charset=utf-8", body)
}

pub async fn llmo_txt() -> impl IntoResponse {
    llms_txt().await
}

pub async fn sitemap_html(State(state): State<AppState>) -> impl IntoResponse {
    let sections = build_sitemap_sections(&state).await;
    let total_urls = sections.iter().map(|section| section.links.len()).sum();

    render(SitemapTemplate {
        sections,
        total_urls,
    })
}

pub async fn sitemap_xml(State(state): State<AppState>) -> impl IntoResponse {
    let sections = build_sitemap_sections(&state).await;

    let mut body = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#,
    );

    for section in sections {
        for entry in section.links {
            push_sitemap_url(
                &mut body,
                &entry.url,
                entry.lastmod.as_deref(),
                &entry.changefreq,
                &entry.priority,
            );
        }
    }

    body.push_str("</urlset>\n");

    text_response("application/xml; charset=utf-8", body)
}

async fn build_sitemap_sections(state: &AppState) -> Vec<SitemapSectionView> {
    let insight_pages = fetch_slug_urls(state, "insights", "/insights").await;
    let portfolio_pages = fetch_slug_urls(state, "portfolios", "/case-studies").await;
    let career_pages = fetch_slug_urls(state, "careers", "/careers").await;
    let service_area_pages = fetch_service_area_urls(state).await;

    let mut sections = Vec::new();

    sections.push(SitemapSectionView {
        title: String::from("Core Pages"),
        description: String::from("Primary commercial, contact, and company pages."),
        link_count: static_sitemap_pages().len(),
        links: static_sitemap_pages()
            .iter()
            .map(|page| static_page_to_link(page))
            .collect(),
    });

    sections.push(SitemapSectionView {
        title: String::from("Service Pages"),
        description: String::from("Delivery capabilities, platforms, and consulting offers."),
        link_count: static_service_pages().len(),
        links: static_service_pages()
            .iter()
            .map(|page| static_page_to_link(page))
            .collect(),
    });

    if !portfolio_pages.is_empty() {
        let link_count = portfolio_pages.len();
        sections.push(SitemapSectionView {
            title: String::from("Case Studies"),
            description: String::from("Published portfolio and project outcome pages."),
            link_count,
            links: portfolio_pages,
        });
    }

    if !insight_pages.is_empty() {
        let link_count = insight_pages.len();
        sections.push(SitemapSectionView {
            title: String::from("Insights"),
            description: String::from("Published articles, guides, and expertise content."),
            link_count,
            links: insight_pages,
        });
    }

    if !career_pages.is_empty() {
        let link_count = career_pages.len();
        sections.push(SitemapSectionView {
            title: String::from("Careers"),
            description: String::from("Open roles and employer brand pages."),
            link_count,
            links: career_pages,
        });
    }

    sections.push(SitemapSectionView {
        title: String::from("Service Areas"),
        description: String::from(
            "City-focused landing pages for LKProfessionals delivery markets.",
        ),
        link_count: service_area_pages.len() + 1,
        links: std::iter::once(SitemapLinkView {
            title: String::from("Service Areas"),
            url: format!("{SITE_URL}/service-areas"),
            description: String::from(
                "Global service area hub for LKProfessionals delivery markets.",
            ),
            has_lastmod: false,
            lastmod: None,
            lastmod_label: String::new(),
            changefreq: String::from("monthly"),
            priority: String::from("0.78"),
        })
        .chain(service_area_pages.into_iter())
        .collect(),
    });

    sections.push(SitemapSectionView {
        title: String::from("Legal and Discovery"),
        description: String::from("Policies and machine-readable discovery endpoints."),
        link_count: static_legal_and_discovery_pages().len(),
        links: static_legal_and_discovery_pages()
            .iter()
            .map(|page| static_page_to_link(page))
            .collect(),
    });

    sections
}

async fn fetch_service_area_urls(state: &AppState) -> Vec<SitemapLinkView> {
    match sqlx::query_as::<_, SitemapServiceAreaRecord>(
        r#"
        SELECT
            area_name,
            slug,
            to_char(
                COALESCE(updated_at, published_at, created_at) AT TIME ZONE 'UTC',
                'YYYY-MM-DD"T"HH24:MI:SS"Z"'
            ) AS lastmod
        FROM service_areas
        WHERE published = TRUE
        ORDER BY market_region ASC, sort_order ASC, area_name ASC
        "#,
    )
    .fetch_all(&state.db)
    .await
    {
        Ok(records) => records
            .into_iter()
            .map(|record| SitemapLinkView {
                title: record.area_name.clone(),
                url: format!("{SITE_URL}/service-areas/{}", record.slug),
                description: format!("Service area page for {}.", record.area_name),
                has_lastmod: record.lastmod.is_some(),
                lastmod_label: record.lastmod.clone().unwrap_or_default(),
                lastmod: record.lastmod,
                changefreq: String::from("monthly"),
                priority: String::from("0.80"),
            })
            .collect(),
        Err(error) => {
            eprintln!("Failed to build sitemap section for service_areas: {error}");
            Vec::new()
        }
    }
}

async fn fetch_slug_urls(state: &AppState, table: &str, prefix: &str) -> Vec<SitemapLinkView> {
    let query = format!(
        r#"
        SELECT
            title,
            slug,
            to_char(
                COALESCE(updated_at, published_at, created_at) AT TIME ZONE 'UTC',
                'YYYY-MM-DD"T"HH24:MI:SS"Z"'
            ) AS lastmod
        FROM {table}
        WHERE published = TRUE
        ORDER BY COALESCE(updated_at, published_at, created_at) DESC
        "#
    );

    match sqlx::query_as::<_, SitemapSlugRecord>(&query)
        .fetch_all(&state.db)
        .await
    {
        Ok(records) => records
            .into_iter()
            .map(|record| SitemapLinkView {
                title: record.title,
                url: format!("{SITE_URL}{prefix}/{}", record.slug),
                description: format!("Published page: {}", record.slug.replace('-', " ")),
                has_lastmod: record.lastmod.is_some(),
                lastmod_label: record.lastmod.clone().unwrap_or_default(),
                lastmod: record.lastmod,
                changefreq: String::from(match table {
                    "insights" => "weekly",
                    "portfolios" => "monthly",
                    "careers" => "weekly",
                    _ => "monthly",
                }),
                priority: String::from(match table {
                    "insights" => "0.72",
                    "portfolios" => "0.78",
                    "careers" => "0.65",
                    _ => "0.60",
                }),
            })
            .collect(),
        Err(error) => {
            eprintln!("Failed to build sitemap section for {table}: {error}");
            Vec::new()
        }
    }
}

fn static_sitemap_pages() -> &'static [StaticSitemapPage] {
    &[
        StaticSitemapPage {
            title: "Home",
            path: "/",
            description: "Global software development, SEO, and digital transformation overview.",
            changefreq: "weekly",
            priority: "1.00",
        },
        StaticSitemapPage {
            title: "About",
            path: "/about",
            description: "Company positioning, founder authority, and delivery approach.",
            changefreq: "monthly",
            priority: "0.70",
        },
        StaticSitemapPage {
            title: "Services",
            path: "/services",
            description: "Service overview covering software, SEO, cloud, and automation.",
            changefreq: "weekly",
            priority: "0.95",
        },
        StaticSitemapPage {
            title: "Case Studies",
            path: "/case-studies",
            description: "Project delivery examples and portfolio outcomes.",
            changefreq: "weekly",
            priority: "0.88",
        },
        StaticSitemapPage {
            title: "Industries",
            path: "/industries",
            description: "Industries and business environments LKProfessionals supports.",
            changefreq: "monthly",
            priority: "0.82",
        },
        StaticSitemapPage {
            title: "Insights",
            path: "/insights",
            description: "Articles, guides, and topical expertise content.",
            changefreq: "daily",
            priority: "0.90",
        },
        StaticSitemapPage {
            title: "Careers",
            path: "/careers",
            description: "Open roles and company careers information.",
            changefreq: "weekly",
            priority: "0.60",
        },
        StaticSitemapPage {
            title: "Why Work at LKProfessionals",
            path: "/careers/why-work-at-lkprofessionals",
            description: "Employer brand and candidate positioning page.",
            changefreq: "monthly",
            priority: "0.52",
        },
        StaticSitemapPage {
            title: "Contact",
            path: "/contact",
            description: "Commercial contact page and lead qualification flow.",
            changefreq: "monthly",
            priority: "0.86",
        },
        StaticSitemapPage {
            title: "FAQ",
            path: "/faq",
            description: "Answers to service, delivery, and engagement questions.",
            changefreq: "monthly",
            priority: "0.68",
        },
        StaticSitemapPage {
            title: "Request Quote",
            path: "/request-quote",
            description: "Project intake form for qualified quote requests.",
            changefreq: "monthly",
            priority: "0.84",
        },
    ]
}

fn static_service_pages() -> &'static [StaticSitemapPage] {
    &[
        StaticSitemapPage {
            title: "Web Development",
            path: "/services/web-development",
            description: "Web platforms, marketing sites, and conversion-focused builds.",
            changefreq: "monthly",
            priority: "0.92",
        },
        StaticSitemapPage {
            title: "Mobile App Development",
            path: "/services/mobile-app-development",
            description: "Native and cross-platform app delivery.",
            changefreq: "monthly",
            priority: "0.85",
        },
        StaticSitemapPage {
            title: "Custom Software Development",
            path: "/services/custom-software-development",
            description: "Business systems, internal tools, and tailored platforms.",
            changefreq: "monthly",
            priority: "0.92",
        },
        StaticSitemapPage {
            title: "Software Development",
            path: "/services/software-development",
            description: "General software engineering and delivery support.",
            changefreq: "monthly",
            priority: "0.88",
        },
        StaticSitemapPage {
            title: "Digital Marketing",
            path: "/services/digital-marketing",
            description: "Growth campaigns, acquisition support, and performance marketing.",
            changefreq: "monthly",
            priority: "0.84",
        },
        StaticSitemapPage {
            title: "SEO Search Growth",
            path: "/services/seo-search-growth",
            description: "SEO strategy, technical SEO, and search growth programs.",
            changefreq: "monthly",
            priority: "0.94",
        },
        StaticSitemapPage {
            title: "Hosting, Domain, and Cloud Services",
            path: "/services/hosting-domain-cloud-services",
            description: "Hosting, domains, cloud setup, and ongoing infrastructure support.",
            changefreq: "monthly",
            priority: "0.76",
        },
        StaticSitemapPage {
            title: "AI Automation Solutions",
            path: "/services/ai-automation-solutions",
            description: "Workflow automation and AI-enabled delivery improvements.",
            changefreq: "monthly",
            priority: "0.90",
        },
        StaticSitemapPage {
            title: "IT Consultation and Digital Transformation",
            path: "/services/it-consultation-digital-transformation",
            description: "Advisory services for modernization, architecture, and planning.",
            changefreq: "monthly",
            priority: "0.82",
        },
    ]
}

fn static_legal_and_discovery_pages() -> &'static [StaticSitemapPage] {
    &[
        StaticSitemapPage {
            title: "Terms and Conditions",
            path: "/terms-and-conditions",
            description: "Commercial terms, project governance, and usage conditions.",
            changefreq: "yearly",
            priority: "0.30",
        },
        StaticSitemapPage {
            title: "Service Level Agreement",
            path: "/service-level-agreement",
            description: "Support commitments and service expectations.",
            changefreq: "yearly",
            priority: "0.30",
        },
        StaticSitemapPage {
            title: "Refund Policy",
            path: "/refund-policy",
            description: "Refund and billing policy page.",
            changefreq: "yearly",
            priority: "0.25",
        },
        StaticSitemapPage {
            title: "Privacy Policy",
            path: "/privacy-policy",
            description: "Data collection, handling, and privacy commitments.",
            changefreq: "yearly",
            priority: "0.25",
        },
        StaticSitemapPage {
            title: "Cookie Policy",
            path: "/cookie-policy",
            description: "Cookie usage and browser-side data policy.",
            changefreq: "yearly",
            priority: "0.25",
        },
        StaticSitemapPage {
            title: "XML Sitemap",
            path: "/sitemap.xml",
            description: "Machine-readable sitemap for search engines.",
            changefreq: "weekly",
            priority: "0.20",
        },
        StaticSitemapPage {
            title: "HTML Sitemap",
            path: "/sitemap.html",
            description: "Human-readable sitemap with dynamic content sections.",
            changefreq: "weekly",
            priority: "0.32",
        },
        StaticSitemapPage {
            title: "Robots.txt",
            path: "/robots.txt",
            description: "Crawler directives and sitemap references.",
            changefreq: "monthly",
            priority: "0.10",
        },
        StaticSitemapPage {
            title: "LLMs.txt",
            path: "/llms.txt",
            description: "Language-model guidance and preferred citation targets.",
            changefreq: "monthly",
            priority: "0.12",
        },
    ]
}

fn static_page_to_link(page: &StaticSitemapPage) -> SitemapLinkView {
    SitemapLinkView {
        title: String::from(page.title),
        url: format!("{SITE_URL}{}", page.path),
        description: String::from(page.description),
        has_lastmod: false,
        lastmod: None,
        lastmod_label: String::new(),
        changefreq: String::from(page.changefreq),
        priority: String::from(page.priority),
    }
}

fn push_sitemap_url(
    body: &mut String,
    location: &str,
    lastmod: Option<&str>,
    changefreq: &str,
    priority: &str,
) {
    body.push_str("  <url>\n");
    body.push_str("    <loc>");
    body.push_str(&xml_escape(location));
    body.push_str("</loc>\n");

    if let Some(lastmod) = lastmod {
        body.push_str("    <lastmod>");
        body.push_str(&xml_escape(lastmod));
        body.push_str("</lastmod>\n");
    }

    body.push_str("    <changefreq>");
    body.push_str(&xml_escape(changefreq));
    body.push_str("</changefreq>\n");
    body.push_str("    <priority>");
    body.push_str(&xml_escape(priority));
    body.push_str("</priority>\n");
    body.push_str("  </url>\n");
}

fn xml_escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn text_response(content_type: &'static str, body: String) -> Response {
    ([(header::CONTENT_TYPE, content_type)], body).into_response()
}
