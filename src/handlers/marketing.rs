use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::{
    render::render,
    templates::{MarketingLandingTemplate, MarketingLandingView},
};

#[derive(Debug, Clone)]
pub struct MarketingStat {
    pub value: String,
    pub label: String,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct MarketingBullet {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct MarketingLink {
    pub label: String,
    pub href: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct MarketingFaq {
    pub question: String,
    pub answer: String,
}

#[derive(Debug, Clone)]
pub struct MarketingTableRow {
    pub topic: String,
    pub lkprofessionals: String,
    pub alternative: String,
}

#[derive(Debug, Clone)]
pub struct MarketingLandingPage {
    pub page_type: String,
    pub title: String,
    pub meta_title: String,
    pub meta_description: String,
    pub canonical_path: String,
    pub eyebrow: String,
    pub hero_title: String,
    pub hero_description: String,
    pub direct_answer: String,
    pub positioning: String,
    pub proof_statement: String,
    pub primary_cta_label: String,
    pub primary_cta_href: String,
    pub secondary_cta_label: String,
    pub secondary_cta_href: String,
    pub tertiary_cta_label: String,
    pub tertiary_cta_href: String,
    pub stats: Vec<MarketingStat>,
    pub buyer_signals: Vec<MarketingBullet>,
    pub deliverables: Vec<MarketingBullet>,
    pub comparison_rows: Vec<MarketingTableRow>,
    pub faqs: Vec<MarketingFaq>,
    pub related_links: Vec<MarketingLink>,
}

#[derive(Debug, Clone, Copy)]
pub struct MarketingSitemapPage {
    pub title: &'static str,
    pub path: &'static str,
    pub description: &'static str,
    pub changefreq: &'static str,
    pub priority: &'static str,
}

const MARKETING_SITEMAP_PAGES: &[MarketingSitemapPage] = &[
    MarketingSitemapPage {
        title: "Custom Software Development Company",
        path: "/solutions/custom-software-development-company",
        description: "Global custom software development landing page.",
        changefreq: "monthly",
        priority: "0.92",
    },
    MarketingSitemapPage {
        title: "Web Development Company",
        path: "/solutions/web-development-company",
        description: "International web development company landing page.",
        changefreq: "monthly",
        priority: "0.90",
    },
    MarketingSitemapPage {
        title: "Software Development Company for Startups",
        path: "/solutions/software-development-company-for-startups",
        description: "Startup software development delivery page.",
        changefreq: "monthly",
        priority: "0.88",
    },
    MarketingSitemapPage {
        title: "Offshore Software Development Company",
        path: "/solutions/offshore-software-development-company",
        description: "Offshore software development and global delivery page.",
        changefreq: "monthly",
        priority: "0.94",
    },
    MarketingSitemapPage {
        title: "Remote Software Development Team",
        path: "/solutions/remote-software-development-team",
        description: "Remote development team engagement page.",
        changefreq: "monthly",
        priority: "0.90",
    },
    MarketingSitemapPage {
        title: "Laravel Development Company",
        path: "/solutions/laravel-development-company",
        description: "Laravel application development landing page.",
        changefreq: "monthly",
        priority: "0.84",
    },
    MarketingSitemapPage {
        title: "Rust Development Company",
        path: "/solutions/rust-development-company",
        description: "Rust software engineering landing page.",
        changefreq: "monthly",
        priority: "0.84",
    },
    MarketingSitemapPage {
        title: "SaaS Development Company",
        path: "/solutions/saas-development-company",
        description: "SaaS product design and development page.",
        changefreq: "monthly",
        priority: "0.88",
    },
    MarketingSitemapPage {
        title: "eCommerce Development Company",
        path: "/solutions/ecommerce-development-company",
        description: "eCommerce platform delivery landing page.",
        changefreq: "monthly",
        priority: "0.86",
    },
    MarketingSitemapPage {
        title: "SEO Company for Small Businesses",
        path: "/solutions/seo-company-for-small-businesses",
        description: "SEO services for small businesses and SMEs.",
        changefreq: "monthly",
        priority: "0.88",
    },
    MarketingSitemapPage {
        title: "AI SEO and GEO Optimization Services",
        path: "/solutions/ai-seo-geo-optimization-services",
        description: "AI search visibility, GEO, and AEO landing page.",
        changefreq: "monthly",
        priority: "0.92",
    },
    MarketingSitemapPage {
        title: "Business Automation Software Development",
        path: "/solutions/business-automation-software-development",
        description: "Business automation systems and workflow software page.",
        changefreq: "monthly",
        priority: "0.88",
    },
    MarketingSitemapPage {
        title: "POS System Development",
        path: "/solutions/pos-system-development",
        description: "Point-of-sale system development landing page.",
        changefreq: "monthly",
        priority: "0.82",
    },
    MarketingSitemapPage {
        title: "ERP Software Development",
        path: "/solutions/erp-software-development",
        description: "ERP design and development landing page.",
        changefreq: "monthly",
        priority: "0.84",
    },
    MarketingSitemapPage {
        title: "Software Development Cost Guide",
        path: "/pricing/software-development-cost-guide",
        description: "Pricing and estimate guide for custom software projects.",
        changefreq: "monthly",
        priority: "0.84",
    },
    MarketingSitemapPage {
        title: "SEO Pricing Guide",
        path: "/pricing/seo-pricing-guide",
        description: "SEO, GEO, and growth retainer pricing guide.",
        changefreq: "monthly",
        priority: "0.80",
    },
    MarketingSitemapPage {
        title: "Offshore Development vs Local Agency",
        path: "/compare/offshore-development-vs-local-agency",
        description: "Comparison page for offshore vs local software partners.",
        changefreq: "monthly",
        priority: "0.82",
    },
    MarketingSitemapPage {
        title: "Freelancer vs Software Development Company",
        path: "/compare/freelancer-vs-software-development-company",
        description: "Comparison page for freelancers versus software companies.",
        changefreq: "monthly",
        priority: "0.78",
    },
    MarketingSitemapPage {
        title: "USA Software Development Company",
        path: "/regions/usa-software-development-company",
        description: "US-facing offshore software development landing page.",
        changefreq: "monthly",
        priority: "0.82",
    },
    MarketingSitemapPage {
        title: "UK Software Development Company",
        path: "/regions/uk-software-development-company",
        description: "UK-facing offshore software development landing page.",
        changefreq: "monthly",
        priority: "0.82",
    },
    MarketingSitemapPage {
        title: "Canada Software Development Company",
        path: "/regions/canada-software-development-company",
        description: "Canada-facing offshore software development landing page.",
        changefreq: "monthly",
        priority: "0.80",
    },
    MarketingSitemapPage {
        title: "Australia Software Development Company",
        path: "/regions/australia-software-development-company",
        description: "Australia-facing offshore software development landing page.",
        changefreq: "monthly",
        priority: "0.80",
    },
    MarketingSitemapPage {
        title: "Europe Software Development Company",
        path: "/regions/europe-software-development-company",
        description: "Europe-facing offshore software development landing page.",
        changefreq: "monthly",
        priority: "0.80",
    },
    MarketingSitemapPage {
        title: "Middle East Software Development Company",
        path: "/regions/middle-east-software-development-company",
        description: "Middle East-facing software development landing page.",
        changefreq: "monthly",
        priority: "0.80",
    },
];

pub fn marketing_sitemap_pages() -> &'static [MarketingSitemapPage] {
    MARKETING_SITEMAP_PAGES
}

pub async fn solution_page(Path(slug): Path<String>) -> Response {
    render_marketing_page(&format!("/solutions/{slug}"), solution_page_content(&slug))
}

pub async fn pricing_page(Path(slug): Path<String>) -> Response {
    render_marketing_page(&format!("/pricing/{slug}"), pricing_page_content(&slug))
}

pub async fn comparison_page(Path(slug): Path<String>) -> Response {
    render_marketing_page(&format!("/compare/{slug}"), comparison_page_content(&slug))
}

pub async fn region_page(Path(slug): Path<String>) -> Response {
    render_marketing_page(&format!("/regions/{slug}"), region_page_content(&slug))
}

pub async fn free_audit_page() -> Response {
    render_marketing_page("/free-website-seo-audit", Some(free_audit_page_content()))
}

fn render_marketing_page(path: &str, page: Option<MarketingLandingPage>) -> Response {
    match page {
        Some(page) => render(MarketingLandingTemplate {
            view: MarketingLandingView {
                page,
                page_url: format!("https://lkprofessionals.com{path}"),
            },
        })
        .into_response(),
        None => (StatusCode::NOT_FOUND, "Page not found.").into_response(),
    }
}

fn service_solution_page(
    slug: &str,
    title: &str,
    keyword: &str,
    hero_description: &str,
    direct_answer: &str,
    proof_statement: &str,
    deliverables: &[(&str, &str)],
    related: &[(&str, &str, &str)],
) -> MarketingLandingPage {
    MarketingLandingPage {
        page_type: "Service".to_string(),
        title: title.to_string(),
        meta_title: format!("{title} | LKProfessionals"),
        meta_description: format!(
            "{keyword} by LKProfessionals, a Sri Lanka-based global delivery partner for software, web, SEO, GEO, automation, and product engineering."
        ),
        canonical_path: format!("/solutions/{slug}"),
        eyebrow: keyword.to_string(),
        hero_title: title.to_string(),
        hero_description: hero_description.to_string(),
        direct_answer: direct_answer.to_string(),
        positioning: "LKProfessionals is positioned as a global software and digital growth partner based in Sri Lanka, serving buyers who want commercial clarity, faster execution, and accountable remote delivery.".to_string(),
        proof_statement: proof_statement.to_string(),
        primary_cta_label: "Book a Free Consultation".to_string(),
        primary_cta_href: "/contact".to_string(),
        secondary_cta_label: "Request a Quote".to_string(),
        secondary_cta_href: "/request-quote".to_string(),
        tertiary_cta_label: "Hire a Remote Development Team".to_string(),
        tertiary_cta_href: "/solutions/remote-software-development-team".to_string(),
        stats: vec![
            stat("Since 2013", "Company history", "Long-running digital delivery company with international-facing execution."),
            stat("Sri Lanka + Global", "Operating model", "Cost-efficient delivery with process quality suited to overseas buyers."),
            stat("Strategy to Support", "Lifecycle coverage", "Scoping, design, development, launch, maintenance, and growth support."),
        ],
        buyer_signals: vec![
            bullet("Best fit buyers", "Startups, SMEs, funded teams, agencies, and operations-heavy businesses that need a dependable external product and engineering partner."),
            bullet("Delivery model", "Remote-first collaboration with milestone planning, weekly progress visibility, QA discipline, and clear documentation."),
            bullet("Trust angle", "Headquartered in Jaffna, Sri Lanka, incorporated as LKProfessionals (Pvt) Ltd., with portfolio work spanning healthcare, travel, property, education, and publishing."),
        ],
        deliverables: deliverables
            .iter()
            .map(|(title, description)| bullet(title, description))
            .collect(),
        comparison_rows: vec![
            table_row("Scoping quality", "Business goals, technical scope, user journey, and post-launch support are defined early.", "Vague requirements often become timeline and budget drift."),
            table_row("Commercial fit", "Sri Lanka-based delivery keeps cost efficient without giving up engineering depth.", "Onshore teams are often materially more expensive for the same first release."),
            table_row("SEO and AI visibility", "Pages, schema, internal links, and answer-style content are built with search and AI discovery in mind.", "Most build-only vendors stop at launch and leave demand generation disconnected."),
            table_row("Long-term ownership", "Maintenance, hosting, analytics, automation, and iteration can stay with one team.", "Teams often need multiple vendors after launch."),
        ],
        faqs: vec![
            faq("Why choose a Sri Lanka-based global delivery partner?", "Because you can access a cost-efficient engineering team in a compatible time zone band for the UK, Middle East, and Asia-Pacific while still supporting North American and European clients through structured async communication."),
            faq("Does LKProfessionals only work with Sri Lankan companies?", "No. LKProfessionals is based in Sri Lanka and actively positions itself for international buyers who need software, web, SEO, GEO, and automation support."),
            faq("Can LKProfessionals work as a remote team extension?", "Yes. Engagements can be structured as scoped projects, phased builds, retained support, or a remote delivery team model."),
        ],
        related_links: related
            .iter()
            .map(|(label, href, description)| link(label, href, description))
            .collect(),
    }
}

fn region_market_page(
    slug: &str,
    title: &str,
    market_label: &str,
    timezone_label: &str,
    competition_label: &str,
) -> MarketingLandingPage {
    MarketingLandingPage {
        page_type: "Region".to_string(),
        title: title.to_string(),
        meta_title: format!("{title} | LKProfessionals"),
        meta_description: format!(
            "{title} from LKProfessionals, a Sri Lanka-based global software partner for {market_label} buyers who need offshore development, SEO, and automation support."
        ),
        canonical_path: format!("/regions/{slug}"),
        eyebrow: format!("{market_label} Delivery"),
        hero_title: format!("{title} from a Sri Lanka-based global partner."),
        hero_description: format!(
            "LKProfessionals helps {market_label} companies reduce delivery cost pressure, move faster on digital projects, and keep one accountable team across software, web, SEO, and automation work."
        ),
        direct_answer: format!(
            "If your team in {market_label} needs a reliable offshore software development company, LKProfessionals is positioned as a premium Sri Lanka-based partner for custom software, websites, SaaS, automation, and AI-search-ready SEO delivery."
        ),
        positioning: format!(
            "The value proposition for {market_label} buyers is simple: lower delivery overhead than many onshore providers, direct communication, strong commercial scoping, and a technology stack that can support modern web, custom software, product, and growth initiatives."
        ),
        proof_statement: "The public portfolio already covers healthcare, travel, property services, education, and publishing projects across multiple markets, which gives buyers execution proof beyond generic sales copy.".to_string(),
        primary_cta_label: "Start Your Project".to_string(),
        primary_cta_href: "/request-quote".to_string(),
        secondary_cta_label: "Book a Free Consultation".to_string(),
        secondary_cta_href: "/contact".to_string(),
        tertiary_cta_label: "View Case Studies".to_string(),
        tertiary_cta_href: "/case-studies".to_string(),
        stats: vec![
            stat("Based in Sri Lanka", "Global operating model", "Positioned for offshore delivery without looking like a low-trust outsourcing middleman."),
            stat(timezone_label, "Timezone compatibility", "Structured communication patterns work across global markets."),
            stat(competition_label, "Commercial reason to evaluate", "Better cost-to-seniority fit than many local agency models."),
        ],
        buyer_signals: vec![
            bullet("What international buyers usually want", "Clear scope, realistic timelines, reliable communication, maintainable code, and the ability to expand into SEO, automation, support, or product growth after launch."),
            bullet("What LKProfessionals emphasizes", "Direct founder-led credibility, company history since 2013, registered-company positioning, portfolio proof, and one team across design, development, SEO, and ongoing support."),
            bullet("Where to go next", "Buyers comparing partners should review service pages, case studies, region pages, and the pricing/comparison content before requesting a quote."),
        ],
        deliverables: vec![
            bullet("Software delivery", "Custom applications, SaaS platforms, dashboards, internal systems, and API-driven workflows."),
            bullet("Web and conversion work", "High-performance websites, landing pages, eCommerce builds, and CRO-focused page systems."),
            bullet("Growth infrastructure", "SEO, GEO, AEO, content structure, analytics setup, hosting, maintenance, and support."),
        ],
        comparison_rows: vec![
            table_row("Cost structure", "Sri Lanka-based delivery is typically more cost efficient than many onshore options.", "Local market rates can make experimentation and iteration more expensive."),
            table_row("Scope flexibility", "One partner can cover discovery, build, launch, SEO, and support.", "Teams often split strategy, development, and marketing across multiple vendors."),
            table_row("International readiness", "Positioned specifically for overseas buyers comparing offshore options.", "Many local firms market locally first and only incidentally support global clients."),
            table_row("Proof path", "Related service pages, case studies, FAQs, and estimate guides are built to support due diligence.", "Buyers often need to ask basic qualification questions before trust is established."),
        ],
        faqs: vec![
            faq(&format!("Why would a {market_label} business hire a Sri Lanka-based software partner?"), "To improve cost efficiency, keep access to experienced implementation support, and work with a team that can cover software, web, SEO, automation, hosting, and maintenance under one commercial relationship."),
            faq("Does LKProfessionals offer only offshore execution?", "No. LKProfessionals can support strategy, discovery, audits, planning, and phased implementation in addition to full remote delivery."),
            faq("Can you support long-term maintenance after launch?", "Yes. Hosting, support, SEO iteration, content growth, and software maintenance can continue after launch."),
        ],
        related_links: vec![
            link("Offshore Software Development Company", "/solutions/offshore-software-development-company", "Core commercial page for buyers comparing global delivery partners."),
            link("Remote Development Team", "/solutions/remote-software-development-team", "Team-extension option for companies that need ongoing delivery capacity."),
            link("Software Pricing Guide", "/pricing/software-development-cost-guide", "Estimate guide that helps buyers understand commercial scope."),
            link("Case Studies", "/case-studies", "Execution proof across multiple sectors and markets."),
        ],
    }
}

fn comparison_content(
    slug: &str,
    title: &str,
    answer: &str,
    rows: &[(&str, &str, &str)],
) -> MarketingLandingPage {
    MarketingLandingPage {
        page_type: "Comparison".to_string(),
        title: title.to_string(),
        meta_title: format!("{title} | LKProfessionals"),
        meta_description: format!(
            "{title} explained with direct answers, comparison criteria, and the best-fit buying path for software, web, SEO, and offshore delivery."
        ),
        canonical_path: format!("/compare/{slug}"),
        eyebrow: "Comparison Guide".to_string(),
        hero_title: title.to_string(),
        hero_description: "A buyer-focused comparison page built for commercial search, AI summaries, and quicker decision-making.".to_string(),
        direct_answer: answer.to_string(),
        positioning: "The best comparison pages do not force a false winner. They show where each option fits, where it breaks, and what a serious buyer should evaluate before committing budget.".to_string(),
        proof_statement: "LKProfessionals uses these comparison pages to support qualified buying decisions, not to overwhelm visitors with sales copy.".to_string(),
        primary_cta_label: "Request a Quote".to_string(),
        primary_cta_href: "/request-quote".to_string(),
        secondary_cta_label: "Book a Free Consultation".to_string(),
        secondary_cta_href: "/contact".to_string(),
        tertiary_cta_label: "Review Service Pages".to_string(),
        tertiary_cta_href: "/services".to_string(),
        stats: vec![
            stat("Decision Support", "Page purpose", "Built to reduce buying friction and clarify the right fit."),
            stat("Search + AI Friendly", "Content style", "Short direct answers, tables, FAQs, and linked proof."),
            stat("Commercial Intent", "Best next step", "Move qualified buyers toward a quote, consultation, or audit."),
        ],
        buyer_signals: vec![
            bullet("Why these searches happen", "Buyers usually want to reduce risk around cost, communication, speed, accountability, and long-term support."),
            bullet("What to evaluate", "Commercial clarity, ownership, quality control, stack fit, process maturity, and post-launch support."),
            bullet("What LKProfessionals adds", "A Sri Lanka-based global delivery model that can bridge software, web, SEO, automation, and operational support."),
        ],
        deliverables: vec![
            bullet("Comparison table", "A clear side-by-side view that AI engines and human buyers can quote quickly."),
            bullet("Direct answer block", "A concise answer before longer explanation to improve AEO and GEO performance."),
            bullet("Next-step links", "Relevant service, case study, pricing, and contact paths to keep buying momentum moving."),
        ],
        comparison_rows: rows
            .iter()
            .map(|(topic, lkprofessionals, alternative)| table_row(topic, lkprofessionals, alternative))
            .collect(),
        faqs: vec![
            faq("Why does comparison content help conversion?", "Because it addresses objections before the buyer needs to ask them in a sales call."),
            faq("Why does comparison content help AI search visibility?", "Because concise answers, tables, and FAQs are easier for answer engines to summarize, compare, and cite."),
            faq("What should a buyer do after reading a comparison page?", "Review the most relevant service page, proof page, and pricing guide, then request a quote or consultation with enough project context to qualify the opportunity."),
        ],
        related_links: vec![
            link("Software Development Cost Guide", "/pricing/software-development-cost-guide", "Estimate ranges, project variables, and budget expectations."),
            link("Offshore Development Company", "/solutions/offshore-software-development-company", "Commercial landing page for buyers evaluating global delivery."),
            link("Case Studies", "/case-studies", "Proof that supports comparison-stage buying decisions."),
            link("Free Website and SEO Audit", "/free-website-seo-audit", "Low-friction lead capture path for buyers not yet ready for a full quote."),
        ],
    }
}

fn pricing_content(
    slug: &str,
    title: &str,
    direct_answer: &str,
    rows: &[(&str, &str, &str)],
) -> MarketingLandingPage {
    MarketingLandingPage {
        page_type: "Pricing".to_string(),
        title: title.to_string(),
        meta_title: format!("{title} | LKProfessionals"),
        meta_description: format!(
            "{title} from LKProfessionals with buyer-friendly guidance on budgets, scope, delivery variables, and the next step for qualified quotes."
        ),
        canonical_path: format!("/pricing/{slug}"),
        eyebrow: "Pricing Guide".to_string(),
        hero_title: title.to_string(),
        hero_description: "A commercial estimate guide built to qualify serious buyers and reduce low-context pricing requests.".to_string(),
        direct_answer: direct_answer.to_string(),
        positioning: "Great pricing pages do not promise fake fixed fees. They explain what changes price, which delivery model fits, and what information a serious buyer should share to get a useful quote.".to_string(),
        proof_statement: "LKProfessionals uses estimate-guided content to improve lead quality, reduce vague pricing requests, and make the sales conversation more efficient.".to_string(),
        primary_cta_label: "Request a Quote".to_string(),
        primary_cta_href: "/request-quote".to_string(),
        secondary_cta_label: "Book a Free Consultation".to_string(),
        secondary_cta_href: "/contact".to_string(),
        tertiary_cta_label: "Free Website / SEO Audit".to_string(),
        tertiary_cta_href: "/free-website-seo-audit".to_string(),
        stats: vec![
            stat("Estimate-First", "Qualification approach", "Budget guidance before custom scoping."),
            stat("Higher Lead Quality", "Primary CRO gain", "Buyers self-select based on scope and seriousness."),
            stat("Global Delivery", "Commercial angle", "Sri Lanka-based execution can improve budget efficiency."),
        ],
        buyer_signals: vec![
            bullet("What affects price", "Complexity, integrations, content readiness, number of user flows, design depth, compliance needs, SEO scope, and post-launch support expectations."),
            bullet("What a useful quote request includes", "Business goal, target users, required features, launch window, budget direction, internal approval status, and any existing system constraints."),
            bullet("What LKProfessionals can scope", "Web development, SaaS, internal software, ERP, POS, SEO, GEO, automation, maintenance, and ongoing support programs."),
        ],
        deliverables: vec![
            bullet("Budget bands", "Clear ranges that help buyers understand whether the initiative fits their expectations."),
            bullet("Scope variables", "The specific items that increase or reduce delivery cost."),
            bullet("Conversion path", "Every pricing guide points back to contact, quote, audit, and relevant service pages."),
        ],
        comparison_rows: rows
            .iter()
            .map(|(topic, lkprofessionals, alternative)| table_row(topic, lkprofessionals, alternative))
            .collect(),
        faqs: vec![
            faq("Why not show one fixed software price?", "Because custom digital work varies based on scope, risk, integration complexity, and post-launch support needs."),
            faq("Does offshore delivery reduce quality?", "Not by default. A weak process reduces quality. A structured Sri Lanka-based delivery partner can reduce cost while preserving quality and accountability."),
            faq("What is the best next step after reviewing a pricing guide?", "Share a requirement brief through the quote form, then use the consultation call to narrow scope and delivery priorities."),
        ],
        related_links: vec![
            link("Software Development Company", "/services/software-development", "Core service page for broader delivery capability."),
            link("Custom Software Development Company", "/solutions/custom-software-development-company", "Commercial landing page for tailored software work."),
            link("Offshore vs Local Agency", "/compare/offshore-development-vs-local-agency", "Comparison guide for buyers balancing budget and quality."),
            link("Request a Quote", "/request-quote", "Move from estimate guide to scoped commercial inquiry."),
        ],
    }
}

fn free_audit_page_content() -> MarketingLandingPage {
    MarketingLandingPage {
        page_type: "Audit".to_string(),
        title: "Free Website and SEO Audit".to_string(),
        meta_title: "Free Website and SEO Audit | LKProfessionals".to_string(),
        meta_description: "Request a free website and SEO audit from LKProfessionals to uncover conversion, SEO, GEO, AEO, performance, and messaging issues.".to_string(),
        canonical_path: "/free-website-seo-audit".to_string(),
        eyebrow: "Lead Magnet".to_string(),
        hero_title: "Free website and SEO audit for businesses that need clearer growth answers.".to_string(),
        hero_description: "LKProfessionals reviews positioning, technical SEO, AI-search readiness, conversion friction, internal linking, mobile experience, and trust gaps before recommending the next practical move.".to_string(),
        direct_answer: "If your website is underperforming, the fastest low-risk next step is a structured audit. LKProfessionals uses the audit to identify what is hurting rankings, answer-engine visibility, user trust, and lead conversion before any rebuild or retainer recommendation.".to_string(),
        positioning: "This page exists to reduce buying friction. Not every prospect should start with a full project quote. Some should start with evidence.".to_string(),
        proof_statement: "The audit CTA supports both SEO and conversion because it creates a lower-friction entry point for visitors who are interested but not yet ready to approve a build or monthly retainer.".to_string(),
        primary_cta_label: "Request a Free Audit".to_string(),
        primary_cta_href: "/contact".to_string(),
        secondary_cta_label: "Request a Quote".to_string(),
        secondary_cta_href: "/request-quote".to_string(),
        tertiary_cta_label: "Chat on WhatsApp".to_string(),
        tertiary_cta_href: "https://wa.me/94761234321".to_string(),
        stats: vec![
            stat("SEO + GEO + CRO", "Audit coverage", "Technical, content, AI-search, and conversion review in one path."),
            stat("Direct Recommendations", "Expected outcome", "A prioritized list of what to fix, what to keep, and what to defer."),
            stat("Low-Friction Entry", "Lead strategy", "Designed for buyers who need evidence before approving a larger project."),
        ],
        buyer_signals: vec![
            bullet("Best fit", "Businesses with an existing website, SEO campaign, service page library, or lead-generation problem that needs diagnosis."),
            bullet("What the audit can reveal", "Weak metadata, poor internal links, generic messaging, low-trust page structure, schema gaps, slow experience, and weak AI-answer extraction signals."),
            bullet("What happens next", "If the audit finds a clear path, LKProfessionals can scope a rebuild, SEO/GEO program, service-page expansion, or conversion optimization plan."),
        ],
        deliverables: vec![
            bullet("Technical review", "Metadata, crawlability, indexability, schema, semantic structure, image handling, and performance notes."),
            bullet("Commercial review", "Homepage messaging, CTA clarity, buyer path friction, trust gaps, and lead capture weaknesses."),
            bullet("AI visibility review", "Answer-style content readiness, entity clarity, FAQ structure, internal linking, and quoteable summary quality."),
        ],
        comparison_rows: vec![
            table_row("Technical SEO", "Crawl, structure, metadata, schema, and performance issues are flagged.", "Problems stay hidden until rankings or traffic fall further."),
            table_row("Conversion quality", "Messaging, CTA placement, and trust signals are reviewed against buyer intent.", "Traffic may exist while lead quality remains weak."),
            table_row("AI search readiness", "Content is checked for direct answers, FAQs, comparison format, and entity clarity.", "Many sites still publish copy that answer engines cannot quote cleanly."),
            table_row("Next-step clarity", "The audit narrows whether you need SEO, CRO, content, new pages, or a rebuild.", "Teams often spend on the wrong fix first."),
        ],
        faqs: vec![
            faq("Who should request the free audit?", "Businesses that already have a website or SEO footprint and need a clearer diagnosis before approving a larger project."),
            faq("Is the audit only for Sri Lankan companies?", "No. LKProfessionals is based in Sri Lanka and positions the audit for local and international clients."),
            faq("How should I request the audit?", "Use the contact page or quote page and describe the website, market, problem, and the main metric you want to improve."),
        ],
        related_links: vec![
            link("AI SEO and GEO Optimization Services", "/solutions/ai-seo-geo-optimization-services", "If the main issue is search visibility in Google and AI engines."),
            link("Web Development Company", "/solutions/web-development-company", "If the audit points toward a stronger commercial website rebuild."),
            link("Software Development Cost Guide", "/pricing/software-development-cost-guide", "If the audit leads into a larger product or systems project."),
            link("Insights", "/insights", "Thought-leadership content that supports authority and entity depth."),
        ],
    }
}

fn solution_page_content(slug: &str) -> Option<MarketingLandingPage> {
    match slug {
        "custom-software-development-company" => Some(service_solution_page(
            slug,
            "Custom Software Development Company",
            "custom software development company",
            "LKProfessionals designs and builds custom internal systems, customer-facing platforms, SaaS products, portals, ERP workflows, POS solutions, and automation-heavy business software for companies in Sri Lanka and international markets.",
            "A custom software development company should do more than code features. It should translate business operations, bottlenecks, and growth goals into maintainable software that improves how the company works. LKProfessionals is positioned for that exact role.",
            "This page supports buyers looking for a global IT partner based in Sri Lanka that can deliver software systems with commercial clarity rather than generic agency language.",
            &[
                (
                    "Discovery and solution design",
                    "Scoping workshops, feature mapping, user roles, data flow design, and delivery planning.",
                ),
                (
                    "Platform engineering",
                    "Secure web apps, portals, dashboards, internal tools, integrations, and API-connected systems.",
                ),
                (
                    "Post-launch support",
                    "Maintenance, enhancements, hosting, performance review, and roadmap expansion.",
                ),
            ],
            &[
                (
                    "Software Development Services",
                    "/services/software-development",
                    "Broader software delivery capability and process detail.",
                ),
                (
                    "ERP Software Development",
                    "/solutions/erp-software-development",
                    "Operational software path for larger workflow-heavy teams.",
                ),
                (
                    "Case Studies",
                    "/case-studies",
                    "Relevant project proof and commercial delivery examples.",
                ),
                (
                    "Free Website and SEO Audit",
                    "/free-website-seo-audit",
                    "Useful for buyers who want diagnosis before a larger scope.",
                ),
            ],
        )),
        "web-development-company" => Some(service_solution_page(
            slug,
            "Web Development Company",
            "web development company",
            "LKProfessionals builds high-performance websites, landing pages, service-page systems, conversion-focused company websites, and scalable web platforms for local and international clients.",
            "A strong web development company does not just ship pages. It builds a commercially clear website architecture that ranks, converts, explains the service properly, and gives the business room to scale. LKProfessionals is positioned around that full outcome.",
            "This page targets buyers who are comparing a basic design vendor against a serious web partner that understands technical SEO, CRO, content structure, and long-term maintainability.",
            &[
                (
                    "Strategic page architecture",
                    "Homepage, services, industries, case studies, comparison pages, and conversion paths.",
                ),
                (
                    "Responsive front-end delivery",
                    "Fast, accessible, mobile-ready builds with premium presentation.",
                ),
                (
                    "Search and conversion setup",
                    "Metadata, schema, internal linking, CTAs, and answer-style content blocks.",
                ),
            ],
            &[
                (
                    "Web Development Service",
                    "/services/web-development",
                    "Detailed delivery process for websites and landing pages.",
                ),
                (
                    "SEO Search Growth",
                    "/services/seo-search-growth",
                    "Search visibility support for web projects.",
                ),
                (
                    "Software Pricing Guide",
                    "/pricing/software-development-cost-guide",
                    "Estimate guidance for larger digital projects.",
                ),
                (
                    "Request a Quote",
                    "/request-quote",
                    "Move from research to commercial scoping.",
                ),
            ],
        )),
        "software-development-company-for-startups" => Some(service_solution_page(
            slug,
            "Software Development Company for Startups",
            "software development company for startups",
            "LKProfessionals supports startups with MVP planning, SaaS builds, platform engineering, landing pages, product websites, and phased release delivery.",
            "Startups need a software development company that can make sensible tradeoffs, not just produce tickets. LKProfessionals is positioned for founders who need an MVP, a launch plan, and a delivery team that understands commercial pressure.",
            "This page is optimized for startup buyers who want a remote product and engineering partner without enterprise-agency overhead.",
            &[
                (
                    "MVP planning",
                    "Scope the first release around buyer value, not feature excess.",
                ),
                (
                    "Launch-ready product build",
                    "Web apps, dashboards, billing flows, admin tools, and supporting websites.",
                ),
                (
                    "Growth support",
                    "Product iteration, content, SEO, landing pages, and conversion improvements after launch.",
                ),
            ],
            &[
                (
                    "SaaS Development Company",
                    "/solutions/saas-development-company",
                    "Best fit for productized software and subscription platforms.",
                ),
                (
                    "Remote Development Team",
                    "/solutions/remote-software-development-team",
                    "Ongoing team extension for startups post-MVP.",
                ),
                (
                    "Contact",
                    "/contact",
                    "Use when the problem is clear and timing matters.",
                ),
                (
                    "Insights",
                    "/insights",
                    "Authority-building articles around digital strategy and search growth.",
                ),
            ],
        )),
        "offshore-software-development-company" => Some(service_solution_page(
            slug,
            "Offshore Software Development Company",
            "offshore software development company",
            "LKProfessionals serves international buyers as a Sri Lanka-based offshore software development company for custom software, SaaS, websites, SEO, automation, ERP, POS, and support delivery.",
            "The best offshore software development company is not the cheapest vendor on a rate card. It is the team that can communicate clearly, scope well, ship maintainable work, and stay accountable after launch. That is the positioning LKProfessionals is building around.",
            "This is one of the most important global commercial pages in the site because it connects Sri Lanka cost-efficiency with international buyer expectations and serious delivery standards.",
            &[
                (
                    "Offshore project delivery",
                    "Scoped builds with fixed milestones, acceptance criteria, and launch support.",
                ),
                (
                    "Remote team extension",
                    "Dedicated capacity for product, engineering, maintenance, SEO, and growth work.",
                ),
                (
                    "Full-stack commercial support",
                    "Web, software, SEO, GEO, hosting, maintenance, and operational automation.",
                ),
            ],
            &[
                (
                    "Remote Development Team",
                    "/solutions/remote-software-development-team",
                    "Alternative engagement model for longer-running delivery.",
                ),
                (
                    "Offshore vs Local Agency",
                    "/compare/offshore-development-vs-local-agency",
                    "Comparison guide for buyers balancing cost and control.",
                ),
                (
                    "USA Region Page",
                    "/regions/usa-software-development-company",
                    "US-facing commercial positioning.",
                ),
                (
                    "UK Region Page",
                    "/regions/uk-software-development-company",
                    "UK-facing commercial positioning.",
                ),
            ],
        )),
        "remote-software-development-team" => Some(service_solution_page(
            slug,
            "Remote Software Development Team",
            "remote software development team",
            "LKProfessionals can operate as a remote software development team for companies that need ongoing product execution, maintenance capacity, support coverage, or a reliable partner across multiple digital initiatives.",
            "Hiring a remote software development team makes sense when the company needs continuity, not just a one-off build. LKProfessionals can operate as that external team across engineering, web, SEO, and automation work.",
            "This page is designed to convert buyers who are past the awareness stage and are already comparing delivery models.",
            &[
                (
                    "Team extension model",
                    "Dedicated or blended delivery support aligned with roadmap priorities.",
                ),
                (
                    "Reporting and communication",
                    "Milestone visibility, issue tracking, async updates, and structured reviews.",
                ),
                (
                    "Cross-functional support",
                    "Engineering, websites, SEO/GEO, content architecture, hosting, and support.",
                ),
            ],
            &[
                (
                    "Offshore Development Company",
                    "/solutions/offshore-software-development-company",
                    "Best if the buyer is comparing partner models.",
                ),
                (
                    "Freelancer vs Software Company",
                    "/compare/freelancer-vs-software-development-company",
                    "Comparison content for delivery model research.",
                ),
                (
                    "Case Studies",
                    "/case-studies",
                    "Proof that the team can deliver beyond sales positioning.",
                ),
                (
                    "Request a Quote",
                    "/request-quote",
                    "Best next step for qualified buying intent.",
                ),
            ],
        )),
        "laravel-development-company" => Some(service_solution_page(
            slug,
            "Laravel Development Company",
            "Laravel development company",
            "LKProfessionals builds Laravel applications for custom platforms, portals, dashboards, admin systems, APIs, and business workflows that need a proven PHP framework with long-term maintainability.",
            "A Laravel development company should offer more than framework familiarity. It should understand product scope, performance, admin workflow design, and long-term support. LKProfessionals is positioned to combine Laravel engineering with business-first delivery.",
            "This page helps the site rank for technology-specific demand while still linking back to broader software and custom software services.",
            &[
                (
                    "Laravel platform builds",
                    "Admin panels, portals, workflow systems, and custom data applications.",
                ),
                (
                    "API and integration work",
                    "Payment, CRM, ERP, marketing, and operational tool integration.",
                ),
                (
                    "Optimization and support",
                    "Refactors, maintenance, bug fixing, performance review, and feature expansion.",
                ),
            ],
            &[
                (
                    "Custom Software Development Company",
                    "/solutions/custom-software-development-company",
                    "Broader custom systems positioning.",
                ),
                (
                    "Software Development Services",
                    "/services/software-development",
                    "General software engineering delivery path.",
                ),
                (
                    "ERP Software Development",
                    "/solutions/erp-software-development",
                    "Workflow-heavy software path.",
                ),
                (
                    "Contact",
                    "/contact",
                    "Use for stack-specific consultations.",
                ),
            ],
        )),
        "rust-development-company" => Some(service_solution_page(
            slug,
            "Rust Development Company",
            "Rust development company",
            "LKProfessionals uses Rust where performance, reliability, backend safety, systems programming, or a modern high-trust stack matters.",
            "A Rust development company is usually selected for performance-sensitive, reliability-sensitive, or infrastructure-conscious software. LKProfessionals can support Rust-backed systems while still covering the web, product, and operational layers around them.",
            "This page gives the brand a technology-specialist surface that also reinforces credibility with the Rust/Axum stack already present in the site.",
            &[
                (
                    "Rust backend engineering",
                    "APIs, services, system components, and reliability-focused backends.",
                ),
                (
                    "Web platform integration",
                    "Rust services paired with modern front-end and SEO-aware web delivery.",
                ),
                (
                    "Architecture and optimization",
                    "Performance-sensitive design, maintainability, and deployment support.",
                ),
            ],
            &[
                (
                    "Software Development Services",
                    "/services/software-development",
                    "Broader engineering capability.",
                ),
                (
                    "Web Development Company",
                    "/solutions/web-development-company",
                    "Commercial website and platform layer.",
                ),
                (
                    "Case Studies",
                    "/case-studies",
                    "Cross-sector proof and delivery confidence.",
                ),
                (
                    "Start Your Project",
                    "/request-quote",
                    "Scope a Rust-backed software initiative.",
                ),
            ],
        )),
        "saas-development-company" => Some(service_solution_page(
            slug,
            "SaaS Development Company",
            "SaaS development company",
            "LKProfessionals builds SaaS products for startups, SMEs, and service businesses that need subscription platforms, user dashboards, role-based workflows, billing flows, and scalable product architecture.",
            "A SaaS development company should help shape the product around user value, onboarding, retention, and internal manageability, not just engineer a login screen and dashboard. LKProfessionals is positioned to support that fuller product outcome.",
            "This page supports one of the highest-value commercial search themes on the site.",
            &[
                (
                    "Product planning",
                    "MVP scoping, user roles, billing flow, feature prioritization, and admin requirements.",
                ),
                (
                    "SaaS platform engineering",
                    "User accounts, dashboards, workflows, notifications, reporting, and integrations.",
                ),
                (
                    "Go-to-market support",
                    "Landing pages, SEO structure, conversion paths, and launch-ready positioning.",
                ),
            ],
            &[
                (
                    "Software Development Company for Startups",
                    "/solutions/software-development-company-for-startups",
                    "Founder-focused product delivery page.",
                ),
                (
                    "Software Development Cost Guide",
                    "/pricing/software-development-cost-guide",
                    "Estimate ranges for product builds.",
                ),
                (
                    "Remote Development Team",
                    "/solutions/remote-software-development-team",
                    "Scale support after MVP.",
                ),
                (
                    "Case Studies",
                    "/case-studies",
                    "Execution proof to support product buyers.",
                ),
            ],
        )),
        "ecommerce-development-company" => Some(service_solution_page(
            slug,
            "eCommerce Development Company",
            "eCommerce development company",
            "LKProfessionals builds eCommerce stores, product catalogs, conversion-focused checkout experiences, category structures, SEO-ready collection pages, and supporting business systems for online sellers.",
            "An eCommerce development company should improve sales operations, product discovery, conversion rates, and maintainability. LKProfessionals is positioned to deliver the store and the surrounding SEO, automation, and support systems that drive revenue.",
            "This page broadens the site beyond generic web development into transaction-focused commerce intent.",
            &[
                (
                    "Storefront and checkout delivery",
                    "Product pages, carts, checkout flow, and mobile commerce experience.",
                ),
                (
                    "Growth structure",
                    "Collection pages, internal linking, content, promotions, and conversion sections.",
                ),
                (
                    "Operational systems",
                    "Inventory support, order workflows, POS/ERP linkage, and automation.",
                ),
            ],
            &[
                (
                    "Web Development Company",
                    "/solutions/web-development-company",
                    "Core commercial web delivery page.",
                ),
                (
                    "POS System Development",
                    "/solutions/pos-system-development",
                    "Commerce operations support.",
                ),
                (
                    "SEO Company for Small Businesses",
                    "/solutions/seo-company-for-small-businesses",
                    "Organic growth support for commerce brands.",
                ),
                (
                    "Request a Quote",
                    "/request-quote",
                    "Commercial scoping for eCommerce projects.",
                ),
            ],
        )),
        "seo-company-for-small-businesses" => Some(service_solution_page(
            slug,
            "SEO Company for Small Businesses",
            "SEO company for small businesses",
            "LKProfessionals helps small businesses improve search visibility, local discovery, service-page quality, AI-answer readiness, and lead generation without bloated enterprise retainers.",
            "A good SEO company for small businesses should improve page quality, not just promise rankings. LKProfessionals is positioned for SMEs that need technical SEO, stronger service pages, local-market support, and better AI-search visibility.",
            "This page captures SME search intent while connecting SEO to conversion and AI discoverability.",
            &[
                (
                    "Technical SEO",
                    "Metadata, crawlability, schema, page structure, and performance improvements.",
                ),
                (
                    "Commercial content",
                    "Service pages, comparison pages, pricing guides, FAQ blocks, and lead-gen content.",
                ),
                (
                    "AI-search readiness",
                    "GEO, AEO, entity clarity, direct-answer copy, and quoteable summaries.",
                ),
            ],
            &[
                (
                    "SEO Search Growth Service",
                    "/services/seo-search-growth",
                    "Broader SEO service detail.",
                ),
                (
                    "AI SEO and GEO Optimization Services",
                    "/solutions/ai-seo-geo-optimization-services",
                    "AI-search-focused landing page.",
                ),
                (
                    "Free Website and SEO Audit",
                    "/free-website-seo-audit",
                    "Low-friction entry point for SEO buyers.",
                ),
                (
                    "Insights",
                    "/insights",
                    "Thought leadership that strengthens entity authority.",
                ),
            ],
        )),
        "ai-seo-geo-optimization-services" => Some(service_solution_page(
            slug,
            "AI SEO and GEO Optimization Services",
            "AI SEO and GEO optimization services",
            "LKProfessionals helps businesses optimize for Google, Bing, ChatGPT, Perplexity, Gemini, Claude, and answer engines by improving structured content, service-page clarity, schema, internal links, direct answers, and entity-rich site architecture.",
            "AI SEO and GEO optimization services are not separate from strong SEO. They are an extension of high-quality technical SEO, better service-page structure, clearer answers, and stronger entity signals. LKProfessionals is positioning this capability as a commercial service, not a vague trend label.",
            "This page is central to the site's GEO/AEO objective and should attract both traditional SEO buyers and AI-search-aware teams.",
            &[
                (
                    "Answer-engine optimization",
                    "Direct answers, comparison content, FAQs, citeable summaries, and clean question-driven structures.",
                ),
                (
                    "Entity and schema improvements",
                    "Organization, service, FAQ, article, breadcrumb, person, and other structured data layers.",
                ),
                (
                    "Internal linking and content hubs",
                    "Link systems connecting services, industries, case studies, pricing guides, and insights.",
                ),
            ],
            &[
                (
                    "SEO Search Growth Service",
                    "/services/seo-search-growth",
                    "Broader SEO service page.",
                ),
                (
                    "Free Website and SEO Audit",
                    "/free-website-seo-audit",
                    "Entry point for audit-led GEO work.",
                ),
                (
                    "AI Search Growth Package",
                    "/packages/business-seo-offer",
                    "Commercial package page with launch pricing and monthly deliverables.",
                ),
                (
                    "Insights",
                    "/insights",
                    "Authority content that supports AI visibility.",
                ),
                (
                    "Service Areas",
                    "/service-areas",
                    "International market pages built for search and answer intent.",
                ),
            ],
        )),
        "business-automation-software-development" => Some(service_solution_page(
            slug,
            "Business Automation Software Development",
            "business automation software development",
            "LKProfessionals builds automation-oriented software for approvals, customer workflows, lead routing, data handling, reporting, operations, and AI-assisted process improvement.",
            "Business automation software development is usually about reducing manual friction, delays, and inconsistent execution. LKProfessionals is positioned to turn those operational pain points into maintainable systems and workflows.",
            "This page lets the site target a strong commercial pain-point keyword rather than only broad software terms.",
            &[
                (
                    "Workflow automation",
                    "Approvals, assignments, notifications, data movement, and internal process control.",
                ),
                (
                    "System integration",
                    "CRM, ERP, forms, dashboards, billing, and operational tool connections.",
                ),
                (
                    "Reporting and optimization",
                    "Visibility into bottlenecks, lead flow, and operational performance.",
                ),
            ],
            &[
                (
                    "AI Automation Solutions",
                    "/services/ai-automation-solutions",
                    "Broader automation service page.",
                ),
                (
                    "ERP Software Development",
                    "/solutions/erp-software-development",
                    "Operational systems for larger organizations.",
                ),
                (
                    "POS System Development",
                    "/solutions/pos-system-development",
                    "Retail and transaction-oriented workflow systems.",
                ),
                (
                    "Contact",
                    "/contact",
                    "Best next step for automation consulting.",
                ),
            ],
        )),
        "pos-system-development" => Some(service_solution_page(
            slug,
            "POS System Development",
            "POS system development",
            "LKProfessionals develops point-of-sale systems for retail, hospitality, service businesses, and multi-branch operations that need transaction handling, reporting, inventory visibility, and operational control.",
            "POS system development is not just a checkout interface problem. It is a workflow, reporting, inventory, and operations problem. LKProfessionals positions POS work as part of a broader business systems capability.",
            "This page supports commerce and retail software demand while linking naturally into ERP, eCommerce, and automation pages.",
            &[
                (
                    "POS application delivery",
                    "Cashier flows, receipts, pricing logic, user roles, and branch-level handling.",
                ),
                (
                    "Inventory and reporting",
                    "Stock visibility, sales summaries, transaction records, and operational dashboards.",
                ),
                (
                    "Integration options",
                    "Accounting, ERP, eCommerce, and customer system linkage.",
                ),
            ],
            &[
                (
                    "ERP Software Development",
                    "/solutions/erp-software-development",
                    "Connected operational systems for larger workflows.",
                ),
                (
                    "Custom Software Development Company",
                    "/solutions/custom-software-development-company",
                    "Broader business software capability.",
                ),
                (
                    "eCommerce Development Company",
                    "/solutions/ecommerce-development-company",
                    "Online commerce path.",
                ),
                (
                    "Request a Quote",
                    "/request-quote",
                    "Scope a POS or multi-system commerce project.",
                ),
            ],
        )),
        "erp-software-development" => Some(service_solution_page(
            slug,
            "ERP Software Development",
            "ERP software development",
            "LKProfessionals develops ERP-style systems for companies that need centralized operations, reporting, approvals, inventory logic, finance-adjacent workflows, HR coordination, and multi-team visibility.",
            "ERP software development should simplify how the business runs, not create a bulky system nobody wants to use. LKProfessionals positions ERP work around commercial realism, workflow clarity, and maintainable delivery.",
            "This page targets high-intent operations buyers and strengthens the site's authority in serious internal systems work.",
            &[
                (
                    "ERP scoping",
                    "Department workflows, permissions, reporting, approvals, and integration requirements.",
                ),
                (
                    "Operational platform builds",
                    "Modules for inventory, finance-adjacent operations, customer handling, HR, or service delivery.",
                ),
                (
                    "Change support",
                    "Phased rollout, process review, training support, and long-term enhancement.",
                ),
            ],
            &[
                (
                    "Business Automation Software Development",
                    "/solutions/business-automation-software-development",
                    "Workflow improvement page.",
                ),
                (
                    "Custom Software Development Company",
                    "/solutions/custom-software-development-company",
                    "Broader custom systems service.",
                ),
                (
                    "Software Development Cost Guide",
                    "/pricing/software-development-cost-guide",
                    "Commercial expectations for larger systems.",
                ),
                (
                    "Case Studies",
                    "/case-studies",
                    "Execution proof that supports larger-system buying decisions.",
                ),
            ],
        )),
        _ => None,
    }
}

fn pricing_page_content(slug: &str) -> Option<MarketingLandingPage> {
    match slug {
        "software-development-cost-guide" => Some(pricing_content(
            slug,
            "Software Development Cost Guide",
            "The cost of software development depends on complexity, workflow depth, integrations, compliance, design detail, and post-launch support. LKProfessionals uses estimate guidance to help buyers understand the range before a custom scope is prepared.",
            &[
                (
                    "Landing page or simple company site",
                    "Lower-cost commercial range when scope, content, and user flows stay focused.",
                    "Costs rise quickly when content is unclear and scope expands late.",
                ),
                (
                    "Custom business software",
                    "Mid-to-high range depending on workflows, integrations, and user roles.",
                    "Prices are usually driven by operational complexity, not page count.",
                ),
                (
                    "SaaS or multi-role platform",
                    "Higher range due to product logic, billing, onboarding, admin, and growth requirements.",
                    "Buyers often underestimate product, support, and QA needs.",
                ),
                (
                    "Ongoing support",
                    "Can be retained monthly for maintenance, SEO, content, automation, and iteration.",
                    "One-off launches without support usually create future drag.",
                ),
            ],
        )),
        "seo-pricing-guide" => Some(pricing_content(
            slug,
            "SEO Pricing Guide",
            "SEO pricing should reflect the amount of technical work, content work, authority building, conversion review, and AI-search optimization needed. LKProfessionals uses this guide to qualify businesses that want actual growth work instead of low-trust package selling.",
            &[
                (
                    "Technical SEO cleanup",
                    "Good fit for sites with strong offers but weak crawlability, metadata, schema, or performance.",
                    "A one-time fix without content strategy may limit growth.",
                ),
                (
                    "Service page expansion",
                    "Useful when the site lacks enough commercial pages for the actual search demand.",
                    "Publishing thin pages can create volume without results.",
                ),
                (
                    "Ongoing SEO / GEO retainer",
                    "Best when the business needs iteration, content, internal links, AEO, and performance review over time.",
                    "Search growth usually compounds through sustained execution, not one-off activity.",
                ),
                (
                    "Audit-led entry",
                    "Free or low-friction audits help identify whether SEO, CRO, content, or a rebuild is the right first move.",
                    "Buying a retainer before diagnosis often wastes budget.",
                ),
            ],
        )),
        _ => None,
    }
}

fn comparison_page_content(slug: &str) -> Option<MarketingLandingPage> {
    match slug {
        "offshore-development-vs-local-agency" => Some(comparison_content(
            slug,
            "Offshore Development vs Local Agency",
            "Offshore development is usually the stronger choice when the buyer wants better cost efficiency, more implementation flexibility, and access to a broader delivery scope. A local agency may still win when in-person collaboration or strict local procurement rules matter most.",
            &[
                (
                    "Cost efficiency",
                    "Better cost-to-output ratio for many digital projects.",
                    "Often materially higher rates, especially in UK, US, Canada, and Australia.",
                ),
                (
                    "Access to skills",
                    "One partner can cover software, web, SEO, GEO, automation, and support.",
                    "Capabilities may be narrower or require subcontractors.",
                ),
                (
                    "Speed and flexibility",
                    "Can move quickly with remote-first process and async communication.",
                    "Local proximity does not always equal faster delivery.",
                ),
                (
                    "Perceived trust",
                    "Needs strong proof, clear process, and communication discipline.",
                    "Starts with geographic trust but still needs delivery quality.",
                ),
            ],
        )),
        "freelancer-vs-software-development-company" => Some(comparison_content(
            slug,
            "Freelancer vs Software Development Company",
            "A freelancer can be a good fit for a narrow task or a very small project. A software development company is usually better when the work affects business operations, needs multiple disciplines, or requires long-term accountability.",
            &[
                (
                    "Breadth of capability",
                    "Multiple disciplines across software, design, SEO, automation, and support.",
                    "Usually strongest in one area, weaker outside it.",
                ),
                (
                    "Risk and continuity",
                    "Lower delivery risk if one person is unavailable.",
                    "Single-person dependency can become a business risk.",
                ),
                (
                    "Complex project fit",
                    "Better for systems, larger websites, product builds, and retained support.",
                    "Better for bounded implementation tasks.",
                ),
                (
                    "Commercial structure",
                    "Clearer scoping, documentation, and post-launch paths.",
                    "Can be flexible, but process maturity varies widely.",
                ),
            ],
        )),
        _ => None,
    }
}

fn region_page_content(slug: &str) -> Option<MarketingLandingPage> {
    match slug {
        "usa-software-development-company" => Some(region_market_page(
            slug,
            "USA Software Development Company",
            "USA",
            "Async-friendly with structured overlap windows",
            "Lower cost pressure than many US agencies",
        )),
        "uk-software-development-company" => Some(region_market_page(
            slug,
            "UK Software Development Company",
            "UK",
            "Strong timezone overlap from Sri Lanka",
            "More efficient than many UK delivery costs",
        )),
        "canada-software-development-company" => Some(region_market_page(
            slug,
            "Canada Software Development Company",
            "Canada",
            "Async-first support with milestone visibility",
            "Useful when Canadian delivery budgets feel heavy",
        )),
        "australia-software-development-company" => Some(region_market_page(
            slug,
            "Australia Software Development Company",
            "Australia",
            "Comfortable APAC working-hour alignment",
            "More budget flexibility than many Australian agency rates",
        )),
        "europe-software-development-company" => Some(region_market_page(
            slug,
            "Europe Software Development Company",
            "Europe",
            "Structured overlap for Europe plus async continuity",
            "Helpful when EU markets need better delivery efficiency",
        )),
        "middle-east-software-development-company" => Some(region_market_page(
            slug,
            "Middle East Software Development Company",
            "Middle East",
            "Very workable timezone overlap from Sri Lanka",
            "Strong fit for Gulf-region cost and speed comparisons",
        )),
        _ => None,
    }
}

fn stat(value: &str, label: &str, detail: &str) -> MarketingStat {
    MarketingStat {
        value: value.to_string(),
        label: label.to_string(),
        detail: detail.to_string(),
    }
}

fn bullet(title: &str, description: &str) -> MarketingBullet {
    MarketingBullet {
        title: title.to_string(),
        description: description.to_string(),
    }
}

fn faq(question: &str, answer: &str) -> MarketingFaq {
    MarketingFaq {
        question: question.to_string(),
        answer: answer.to_string(),
    }
}

fn table_row(topic: &str, lkprofessionals: &str, alternative: &str) -> MarketingTableRow {
    MarketingTableRow {
        topic: topic.to_string(),
        lkprofessionals: lkprofessionals.to_string(),
        alternative: alternative.to_string(),
    }
}

fn link(label: &str, href: &str, description: &str) -> MarketingLink {
    MarketingLink {
        label: label.to_string(),
        href: href.to_string(),
        description: description.to_string(),
    }
}
