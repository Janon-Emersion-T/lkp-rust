#[derive(Clone, Copy)]
pub struct ServiceCard {
    pub slug: &'static str,
    pub title: &'static str,
    pub short_title: &'static str,
    pub category: &'static str,
    pub summary: &'static str,
    pub icon: &'static str,
}

#[derive(Clone, Copy)]
pub struct ServiceStat {
    pub value: &'static str,
    pub label: &'static str,
    pub note: &'static str,
}

#[derive(Clone, Copy)]
pub struct ServicePoint {
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy)]
pub struct ServiceDeliverable {
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy)]
pub struct ServiceStep {
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy)]
pub struct ServiceFaq {
    pub question: &'static str,
    pub answer: &'static str,
}

pub struct ServiceOverviewContext {
    pub services: Vec<ServiceCard>,
    pub proof_points: Vec<ServicePoint>,
    pub process: Vec<ServiceStep>,
}

pub struct ServicePageContext {
    pub slug: &'static str,
    pub title: &'static str,
    pub meta_title: &'static str,
    pub meta_description: &'static str,
    pub canonical_path: &'static str,
    pub eyebrow: &'static str,
    pub hero_title: &'static str,
    pub hero_description: &'static str,
    pub hero_panel_title: &'static str,
    pub hero_panel_body: &'static str,
    pub hero_checklist: Vec<&'static str>,
    pub primary_cta_label: &'static str,
    pub primary_cta_href: &'static str,
    pub secondary_cta_label: &'static str,
    pub secondary_cta_href: &'static str,
    pub stats: Vec<ServiceStat>,
    pub challenges: Vec<ServicePoint>,
    pub deliverables: Vec<ServiceDeliverable>,
    pub process: Vec<ServiceStep>,
    pub differentiators: Vec<ServicePoint>,
    pub faqs: Vec<ServiceFaq>,
    pub related_services: Vec<ServiceCard>,
}

pub fn services_overview_context() -> ServiceOverviewContext {
    ServiceOverviewContext {
        services: all_service_cards(),
        proof_points: vec![
            ServicePoint {
                title: "Business-first scoping",
                description: "We translate goals, friction, and budget realities into a sensible delivery path before writing a single line of code.",
            },
            ServicePoint {
                title: "One team across the stack",
                description: "Strategy, design direction, engineering, optimization, launch support, and growth iteration stay connected instead of being handed across silos.",
            },
            ServicePoint {
                title: "Built to keep compounding",
                description: "Our service packages prioritize maintainability, speed, search visibility, conversion clarity, and the operational handoff your team actually needs.",
            },
        ],
        process: vec![
            ServiceStep {
                title: "Discover the opportunity",
                description: "We map your current state, constraints, audience, and commercial objectives so the service mix fits the business instead of following a generic playbook.",
            },
            ServiceStep {
                title: "Shape the solution",
                description: "We turn findings into a focused roadmap covering scope, milestones, technical decisions, content needs, and growth priorities.",
            },
            ServiceStep {
                title: "Build with momentum",
                description: "Design, development, optimization, and QA happen with visible progress and practical checkpoints, not long black-box delivery cycles.",
            },
            ServiceStep {
                title: "Launch and improve",
                description: "After release we continue with support, iteration, SEO tuning, marketing calibration, automation improvements, or further product expansion as needed.",
            },
        ],
    }
}

pub fn service_page_context(slug: &str) -> ServicePageContext {
    match slug {
        "web-development" => ServicePageContext {
            slug: "web-development",
            title: "Web Development",
            meta_title: "Web Development Services in Sri Lanka | LKProfessionals",
            meta_description: "Conversion-focused web development for corporate sites, landing pages, portals, and high-performance digital experiences by LKProfessionals.",
            canonical_path: "/services/web-development",
            eyebrow: "Web Development",
            hero_title: "Websites that look sharp, load fast, and turn attention into action.",
            hero_description: "We design and build business websites, campaign landing pages, and content-rich web platforms that balance speed, credibility, SEO readiness, and lead generation.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Brands that need a modern website upgrade, a stronger digital first impression, or a web experience built to support sales and marketing.",
            hero_checklist: vec![
                "Corporate websites and brand refreshes",
                "Landing pages built for campaigns and lead capture",
                "Portals, directories, and content-driven web experiences",
            ],
            primary_cta_label: "Start Your Website Project",
            primary_cta_href: "/contact",
            secondary_cta_label: "Request a Quote",
            secondary_cta_href: "/request-quote",
            stats: vec![
                ServiceStat {
                    value: "Fast",
                    label: "Performance mindset",
                    note: "Lean builds focused on load speed, clarity, and responsiveness.",
                },
                ServiceStat {
                    value: "SEO-ready",
                    label: "Technical foundation",
                    note: "Structured content and search-friendly architecture from day one.",
                },
                ServiceStat {
                    value: "Flexible",
                    label: "Scales with growth",
                    note: "Built to expand into campaigns, integrations, and future features.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Outdated design hurting trust",
                    description: "We replace generic or aging interfaces with a clear visual system that feels current, capable, and credible.",
                },
                ServicePoint {
                    title: "Poor conversion flow",
                    description: "We simplify messaging, hierarchy, and calls to action so visitors know what to do next.",
                },
                ServicePoint {
                    title: "Slow, hard-to-maintain websites",
                    description: "We rebuild around clean structure and scalable content sections that are easier to improve over time.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "UX-led page architecture",
                    description: "Intentional layouts for homepage, service pages, conversion paths, and supporting content.",
                },
                ServiceDeliverable {
                    title: "Responsive front-end development",
                    description: "Polished experiences that hold up across desktop, tablet, and mobile screens.",
                },
                ServiceDeliverable {
                    title: "Content and SEO structure",
                    description: "Metadata, internal linking, information hierarchy, and crawl-friendly page organization.",
                },
                ServiceDeliverable {
                    title: "Launch support",
                    description: "Pre-launch QA, deployment assistance, and practical post-launch stabilization.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Position the site",
                    description: "We define the audience, offer, site structure, and content priorities before production begins.",
                },
                ServiceStep {
                    title: "Design the experience",
                    description: "Wireframes and visual direction align brand personality with usability and conversion goals.",
                },
                ServiceStep {
                    title: "Build and optimize",
                    description: "We implement the front end, tune performance, and test key interactions and layouts.",
                },
                ServiceStep {
                    title: "Launch confidently",
                    description: "Final QA, analytics readiness, and support handoff help the new website go live without chaos.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Design with commercial intent",
                    description: "The work is not just decorative. Each section is expected to support trust, education, or action.",
                },
                ServicePoint {
                    title: "Technical and marketing alignment",
                    description: "Web build decisions consider speed, search, content, and future campaigns together.",
                },
                ServicePoint {
                    title: "Modular page systems",
                    description: "Reusable sections make future edits, new campaigns, and page expansion much easier.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do you build only simple websites?",
                    answer: "No. We handle corporate sites, landing pages, directories, portals, and more complex content-focused web platforms.",
                },
                ServiceFaq {
                    question: "Can you improve an existing website instead of rebuilding everything?",
                    answer: "Yes. If the current site has a solid enough foundation, we can modernize design, restructure pages, and improve performance incrementally.",
                },
                ServiceFaq {
                    question: "Will the website be optimized for search engines?",
                    answer: "Yes. We bake in core technical SEO considerations such as structure, metadata, internal linking, performance, and content hierarchy.",
                },
            ],
            related_services: related_services(&[
                "seo-search-growth",
                "digital-marketing",
                "hosting-domain-cloud-services",
            ]),
        },
        "mobile-app-development" => ServicePageContext {
            slug: "mobile-app-development",
            title: "Mobile App Development",
            meta_title: "Mobile App Development in Sri Lanka | LKProfessionals",
            meta_description: "Mobile app development for Android, iOS, and cross-platform products focused on usability, reliability, and business outcomes.",
            canonical_path: "/services/mobile-app-development",
            eyebrow: "Mobile App Development",
            hero_title: "Mobile apps built for real users, real workflows, and real adoption.",
            hero_description: "We create customer-facing and internal mobile applications that streamline workflows, improve service access, and support growth across Android, iOS, and cross-platform environments.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Businesses launching new digital products, extending a web platform into mobile, or replacing manual operations with app-driven workflows.",
            hero_checklist: vec![
                "Customer apps, booking apps, and service platforms",
                "Internal workflow apps for teams in the field",
                "Cross-platform product builds with room to expand",
            ],
            primary_cta_label: "Plan Your Mobile App",
            primary_cta_href: "/contact",
            secondary_cta_label: "See All Services",
            secondary_cta_href: "/services",
            stats: vec![
                ServiceStat {
                    value: "UX-led",
                    label: "User adoption focus",
                    note: "Interfaces are designed for clarity, speed, and repeat use.",
                },
                ServiceStat {
                    value: "Stable",
                    label: "Production-ready builds",
                    note: "Quality checks and sensible architecture reduce launch risk.",
                },
                ServiceStat {
                    value: "Connected",
                    label: "API and system friendly",
                    note: "Apps are planned to work with web platforms, dashboards, and internal tools.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "App ideas without a product plan",
                    description: "We convert rough concepts into a scoped product path with clear priorities and release logic.",
                },
                ServicePoint {
                    title: "Clunky experiences that users abandon",
                    description: "We improve navigation, task flows, and interaction patterns so the product is easier to use daily.",
                },
                ServicePoint {
                    title: "Disconnected mobile and business systems",
                    description: "We design app features with backend integrations, reporting, and operations in mind.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Product scope and feature mapping",
                    description: "A sensible MVP or phased roadmap grounded in business value and usability.",
                },
                ServiceDeliverable {
                    title: "Interface and flow design",
                    description: "Screen systems that reduce friction and keep complex tasks understandable.",
                },
                ServiceDeliverable {
                    title: "App development and integration",
                    description: "Implementation for mobile platforms with the data connections the product needs.",
                },
                ServiceDeliverable {
                    title: "Testing and launch readiness",
                    description: "QA across devices, key scenarios, and handoff steps before release.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Clarify the use case",
                    description: "We identify users, core actions, dependencies, and what success should look like after launch.",
                },
                ServiceStep {
                    title: "Prototype the experience",
                    description: "Key user journeys are shaped before development so the product direction is deliberate.",
                },
                ServiceStep {
                    title: "Build the app stack",
                    description: "App screens, backend hooks, and business logic are implemented in a staged delivery flow.",
                },
                ServiceStep {
                    title: "Validate and release",
                    description: "We test critical interactions, polish release quality, and support the go-live motion.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Product thinking, not screen counting",
                    description: "We prioritize how the app should behave and what it should unlock for the business.",
                },
                ServicePoint {
                    title: "Strong workflow awareness",
                    description: "Internal operations and customer usage patterns shape architecture and interface choices.",
                },
                ServicePoint {
                    title: "Designed for iteration",
                    description: "We structure products so new releases and feature expansion can happen cleanly.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do you build for Android and iPhone?",
                    answer: "Yes. We can scope for Android, iOS, or cross-platform delivery depending on the product goals and budget.",
                },
                ServiceFaq {
                    question: "Can the app connect with our existing software?",
                    answer: "In many cases, yes. We plan integrations around available APIs, database access, and operational requirements.",
                },
                ServiceFaq {
                    question: "Can you help define the MVP?",
                    answer: "Yes. We often help clients shape the first release so the app launches with the right core value instead of too much complexity.",
                },
            ],
            related_services: related_services(&[
                "software-development",
                "ai-automation-solutions",
                "it-consultation-digital-transformation",
            ]),
        },
        "custom-software-development" => ServicePageContext {
            slug: "custom-software-development",
            title: "Custom Software Development",
            meta_title: "Custom Software Development in Sri Lanka | LKProfessionals",
            meta_description: "Custom software systems for operations, sales, reporting, education, healthcare, logistics, and other business workflows.",
            canonical_path: "/services/custom-software-development",
            eyebrow: "Custom Software Development",
            hero_title: "Tailor-made software for the way your business actually operates.",
            hero_description: "We build custom systems for organizations that have outgrown spreadsheets, fragmented tools, or generic off-the-shelf software that never quite fits.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Teams that need software aligned to their processes, data flow, reporting requirements, and industry-specific realities.",
            hero_checklist: vec![
                "ERP, CRM, POS, inventory, and workflow systems",
                "Sector-specific solutions for education, healthcare, and services",
                "Internal tools replacing manual or duplicate work",
            ],
            primary_cta_label: "Discuss Your Software Needs",
            primary_cta_href: "/contact",
            secondary_cta_label: "Request a Quote",
            secondary_cta_href: "/request-quote",
            stats: vec![
                ServiceStat {
                    value: "Tailored",
                    label: "Fits your operation",
                    note: "Processes, permissions, reports, and workflows are shaped around your business.",
                },
                ServiceStat {
                    value: "Integrated",
                    label: "Connects core systems",
                    note: "We design around data flow between departments, tools, and stakeholders.",
                },
                ServiceStat {
                    value: "Practical",
                    label: "Change management aware",
                    note: "Solutions are scoped to be usable by the team that will run them.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Too many disconnected tools",
                    description: "We consolidate critical workflows into one coordinated system or integration layer.",
                },
                ServicePoint {
                    title: "Manual processes slowing the team",
                    description: "We translate repetitive work into structured flows, automations, and better data visibility.",
                },
                ServicePoint {
                    title: "Generic software that never quite fits",
                    description: "We model permissions, states, reports, and logic around your actual operational needs.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "System discovery and workflow mapping",
                    description: "We document the moving parts before proposing the platform shape.",
                },
                ServiceDeliverable {
                    title: "Role-based product design",
                    description: "Dashboards, forms, and actions are organized around the people using them.",
                },
                ServiceDeliverable {
                    title: "Business logic and reporting",
                    description: "Rules, status tracking, reporting views, and operational controls are built into the product.",
                },
                ServiceDeliverable {
                    title: "Rollout support",
                    description: "We help prepare data, feedback loops, and post-launch refinements after release.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Map the business flow",
                    description: "We identify stakeholders, exceptions, approvals, and the data points the software must handle.",
                },
                ServiceStep {
                    title: "Design the system structure",
                    description: "Modules, roles, dashboards, and reporting requirements are shaped into a sensible solution model.",
                },
                ServiceStep {
                    title: "Develop in milestones",
                    description: "We build incrementally so the software can be validated against real operational expectations.",
                },
                ServiceStep {
                    title: "Adopt and refine",
                    description: "Feedback from daily use informs adjustments that make the system more effective after launch.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Operations-aware software design",
                    description: "We care about how departments, approvals, and exceptions work in practice.",
                },
                ServicePoint {
                    title: "Reporting built into the plan",
                    description: "Decision-making and visibility needs are considered early, not added as an afterthought.",
                },
                ServicePoint {
                    title: "Scope that respects change risk",
                    description: "We phase complex systems realistically so adoption is smoother and delivery stays grounded.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "What kinds of custom systems do you build?",
                    answer: "We work on internal tools, operational platforms, ERPs, CRMs, POS systems, portals, and industry-specific applications.",
                },
                ServiceFaq {
                    question: "Can custom software be built in phases?",
                    answer: "Yes. Many of the strongest projects start with a focused first release and expand as the team adopts the system.",
                },
                ServiceFaq {
                    question: "Do you help improve old manual workflows before development?",
                    answer: "Yes. We usually begin by understanding the workflow so the software solves the right problem instead of digitizing inefficiency.",
                },
            ],
            related_services: related_services(&[
                "software-development",
                "ai-automation-solutions",
                "it-consultation-digital-transformation",
            ]),
        },
        "software-development" => ServicePageContext {
            slug: "software-development",
            title: "Software Development",
            meta_title: "Software Development Services in Sri Lanka | LKProfessionals",
            meta_description: "Software development for platforms, dashboards, portals, and digital products with strong architecture and long-term maintainability.",
            canonical_path: "/services/software-development",
            eyebrow: "Software Development",
            hero_title: "Robust software products built for scale, control, and long-term maintainability.",
            hero_description: "From SaaS-style platforms to internal dashboards and operational portals, we engineer software that balances product usability with solid technical foundations.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Organizations building a serious web-based product, digitizing internal systems, or needing software architecture that can handle future expansion.",
            hero_checklist: vec![
                "Business platforms, portals, and admin dashboards",
                "Internal systems with permissions, workflows, and reporting",
                "Web products expected to grow feature by feature",
            ],
            primary_cta_label: "Build Your Software Platform",
            primary_cta_href: "/contact",
            secondary_cta_label: "See Related Services",
            secondary_cta_href: "/services",
            stats: vec![
                ServiceStat {
                    value: "Structured",
                    label: "Architecture-led",
                    note: "We think in systems, maintainability, and clean growth paths.",
                },
                ServiceStat {
                    value: "Secure",
                    label: "Business-critical ready",
                    note: "Permissions, data handling, and operational reliability are planned deliberately.",
                },
                ServiceStat {
                    value: "Expandable",
                    label: "Supports future modules",
                    note: "The foundation is built so the product can mature instead of collapsing under change.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Software that became hard to extend",
                    description: "We shape more maintainable structures that support new modules and cleaner decision-making.",
                },
                ServicePoint {
                    title: "Products missing admin control or visibility",
                    description: "We build the dashboard, reporting, and operational layers that make software manageable.",
                },
                ServicePoint {
                    title: "Ambitious product ideas without a solid build plan",
                    description: "We turn the vision into milestones, architecture choices, and release priorities.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Platform architecture planning",
                    description: "Roles, modules, data relationships, and integration points defined before heavy build work.",
                },
                ServiceDeliverable {
                    title: "Core product development",
                    description: "Front-end interfaces, backend logic, dashboards, and key workflows implemented coherently.",
                },
                ServiceDeliverable {
                    title: "Operational tooling",
                    description: "Admin panels, permissions, reporting, and controls that help the business run the product.",
                },
                ServiceDeliverable {
                    title: "Stability and handoff support",
                    description: "Testing, fixes, documentation support, and next-phase planning after delivery.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Define the product core",
                    description: "We identify the essential workflows, roles, and data structures the platform depends on.",
                },
                ServiceStep {
                    title: "Establish the system blueprint",
                    description: "Architecture and milestone planning keep the build grounded and extensible.",
                },
                ServiceStep {
                    title: "Implement critical workflows",
                    description: "We build the high-value product paths first, then expand with supporting modules.",
                },
                ServiceStep {
                    title: "Support the next release path",
                    description: "We help stabilize the product and outline the most valuable follow-up improvements.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Engineering discipline with business context",
                    description: "We care about good architecture because it supports product speed, stability, and smarter change.",
                },
                ServicePoint {
                    title: "Operational completeness",
                    description: "A platform is more than the user-facing interface. Admin tooling and visibility matter too.",
                },
                ServicePoint {
                    title: "Designed for the second phase",
                    description: "We build with future releases in mind so success does not become technical debt.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "How is this different from custom software development?",
                    answer: "There is overlap, but this service is positioned around broader software platforms and digital products, while custom software development is more workflow-specific and operationally tailored.",
                },
                ServiceFaq {
                    question: "Can you work on an existing software product?",
                    answer: "Yes. We can assess an existing codebase or product direction and recommend a sensible improvement path.",
                },
                ServiceFaq {
                    question: "Do you build admin dashboards too?",
                    answer: "Yes. Admin workflows, permissions, reporting, and control layers are often a core part of the delivery.",
                },
            ],
            related_services: related_services(&[
                "custom-software-development",
                "mobile-app-development",
                "web-development",
            ]),
        },
        "digital-marketing" => ServicePageContext {
            slug: "digital-marketing",
            title: "Digital Marketing",
            meta_title: "Digital Marketing Services in Sri Lanka | LKProfessionals",
            meta_description: "Digital marketing strategy, campaign execution, content direction, and growth systems for brands that want measurable momentum.",
            canonical_path: "/services/digital-marketing",
            eyebrow: "Digital Marketing",
            hero_title: "Marketing systems that create attention, consistency, and qualified demand.",
            hero_description: "We help businesses improve online visibility and lead flow through focused digital marketing strategy, campaign planning, content direction, and channel execution.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Businesses that need more qualified attention online, clearer campaign direction, and better alignment between content, traffic, and conversion.",
            hero_checklist: vec![
                "Campaign planning across search, social, and content",
                "Brand visibility and lead-generation support",
                "Marketing built to work with your website and offers",
            ],
            primary_cta_label: "Talk About Growth",
            primary_cta_href: "/contact",
            secondary_cta_label: "Explore SEO Services",
            secondary_cta_href: "/services/seo-search-growth",
            stats: vec![
                ServiceStat {
                    value: "Targeted",
                    label: "Audience-aware execution",
                    note: "Campaigns are shaped around real buyer intent and market positioning.",
                },
                ServiceStat {
                    value: "Consistent",
                    label: "Content system focus",
                    note: "We build repeatable messaging instead of one-off bursts of activity.",
                },
                ServiceStat {
                    value: "Aligned",
                    label: "Website-to-campaign fit",
                    note: "Traffic strategy and landing experience are treated as one system.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Online activity without real direction",
                    description: "We create a clearer growth plan that connects audience, channels, offers, and actions.",
                },
                ServicePoint {
                    title: "Weak lead quality from campaigns",
                    description: "We tighten messaging and campaign structure so the right prospects are more likely to engage.",
                },
                ServicePoint {
                    title: "Inconsistent content and brand visibility",
                    description: "We help establish a repeatable marketing rhythm instead of scattered posting or reactive promotions.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Channel and campaign strategy",
                    description: "A practical plan for where to focus attention and what each channel should do.",
                },
                ServiceDeliverable {
                    title: "Content and messaging direction",
                    description: "Clearer offer framing, campaign angles, and communication priorities.",
                },
                ServiceDeliverable {
                    title: "Campaign setup and optimization",
                    description: "Structured execution, performance review, and iteration around what the data shows.",
                },
                ServiceDeliverable {
                    title: "Landing and conversion alignment",
                    description: "Support for the pages and CTA paths that need to convert the traffic being generated.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Audit current visibility",
                    description: "We review channels, messaging, traffic sources, and offer clarity to find the biggest gaps.",
                },
                ServiceStep {
                    title: "Shape the campaign plan",
                    description: "Audience segments, content themes, offers, and channel priorities are aligned into a workable roadmap.",
                },
                ServiceStep {
                    title: "Execute and iterate",
                    description: "Campaigns and content are launched with regular review loops and informed improvements.",
                },
                ServiceStep {
                    title: "Strengthen the funnel",
                    description: "Insights from performance feed into better landing pages, messaging, and budget allocation.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Commercial clarity over vanity activity",
                    description: "The goal is not random reach. It is better-fit traffic, stronger trust, and measurable action.",
                },
                ServicePoint {
                    title: "Integrated with product and website realities",
                    description: "Marketing recommendations reflect the actual offer, site experience, and sales process.",
                },
                ServicePoint {
                    title: "Built for consistency",
                    description: "We prefer repeatable systems over temporary spikes that disappear as soon as campaigns pause.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do you handle only social media marketing?",
                    answer: "No. We can support broader digital marketing across search, content, campaigns, landing pages, and visibility strategy.",
                },
                ServiceFaq {
                    question: "Can this work together with a website redesign?",
                    answer: "Yes. In fact, aligning your web experience with your marketing goals often improves results significantly.",
                },
                ServiceFaq {
                    question: "Do you support lead-generation campaigns?",
                    answer: "Yes. We can help structure campaigns and landing experiences aimed at generating better qualified inquiries.",
                },
            ],
            related_services: related_services(&[
                "seo-search-growth",
                "web-development",
                "ai-automation-solutions",
            ]),
        },
        "seo-search-growth" => ServicePageContext {
            slug: "seo-search-growth",
            title: "SEO & Search Growth",
            meta_title: "SEO Services in Sri Lanka | LKProfessionals Search Growth",
            meta_description: "Technical SEO, content strategy, on-page optimization, and search growth services for businesses that want durable organic visibility.",
            canonical_path: "/services/seo-search-growth",
            eyebrow: "SEO & Search Growth",
            hero_title: "Search growth built on technical strength, content clarity, and sustained momentum.",
            hero_description: "We help businesses improve organic visibility with technical SEO, content planning, local search support, on-page refinement, and performance-led search strategy.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Organizations that want to rank more meaningfully, strengthen local visibility, and turn their website into a better organic acquisition channel.",
            hero_checklist: vec![
                "Technical SEO and crawl health improvements",
                "On-page structure, content priorities, and internal linking",
                "Local search visibility and long-term organic growth support",
            ],
            primary_cta_label: "Improve Your Search Visibility",
            primary_cta_href: "/contact",
            secondary_cta_label: "See Web Development",
            secondary_cta_href: "/services/web-development",
            stats: vec![
                ServiceStat {
                    value: "Technical",
                    label: "Foundation first",
                    note: "Search performance starts with structure, speed, and indexability.",
                },
                ServiceStat {
                    value: "Intent-led",
                    label: "Content that matches queries",
                    note: "We align pages and content direction with how people actually search.",
                },
                ServiceStat {
                    value: "Durable",
                    label: "Organic momentum focus",
                    note: "The aim is compounding visibility, not short-lived tricks.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Pages not ranking for meaningful terms",
                    description: "We refine site structure, topical focus, and page intent to improve relevance.",
                },
                ServicePoint {
                    title: "Technical issues undermining visibility",
                    description: "We identify speed, metadata, indexing, architecture, and structural problems that weaken search performance.",
                },
                ServicePoint {
                    title: "Content growth without search strategy",
                    description: "We define what to create, optimize, consolidate, or strengthen based on opportunity and intent.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "SEO audit and prioritization",
                    description: "A practical view of what is holding the site back and what should happen first.",
                },
                ServiceDeliverable {
                    title: "On-page and structural optimization",
                    description: "Improvements to page hierarchy, metadata, internal links, and target-page clarity.",
                },
                ServiceDeliverable {
                    title: "Content opportunity planning",
                    description: "Guidance on pages to build or improve in order to capture stronger search demand.",
                },
                ServiceDeliverable {
                    title: "Ongoing measurement and tuning",
                    description: "Iterative search growth support informed by rankings, traffic trends, and page performance.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Audit the search foundation",
                    description: "We review the technical and content landscape to see where visibility is being blocked.",
                },
                ServiceStep {
                    title: "Prioritize high-impact fixes",
                    description: "We separate urgent technical work from medium-term content and structure opportunities.",
                },
                ServiceStep {
                    title: "Implement optimization work",
                    description: "Technical, on-page, and content improvements are rolled out in a focused sequence.",
                },
                ServiceStep {
                    title: "Expand what performs",
                    description: "We use results and query behavior to deepen the content footprint and improve compounding growth.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "SEO paired with site quality",
                    description: "Search recommendations are grounded in performance, UX, and conversion realities.",
                },
                ServicePoint {
                    title: "No gimmick-heavy playbook",
                    description: "We focus on the fundamentals that create durable visibility instead of short-term shortcuts.",
                },
                ServicePoint {
                    title: "Built to support broader marketing",
                    description: "SEO becomes stronger when it connects with your web structure, content, and offer strategy.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do you offer technical SEO only, or content SEO too?",
                    answer: "Both. We can support technical cleanup, on-page optimization, local search, and content direction for organic growth.",
                },
                ServiceFaq {
                    question: "Can you help a new website rank from the beginning?",
                    answer: "Yes. Starting with a strong structure and content plan usually creates a better search trajectory.",
                },
                ServiceFaq {
                    question: "Is SEO useful if we already run ads?",
                    answer: "Yes. SEO and paid acquisition can complement each other, especially when the site structure and landing pages are aligned.",
                },
            ],
            related_services: related_services(&[
                "web-development",
                "digital-marketing",
                "hosting-domain-cloud-services",
            ]),
        },
        "hosting-domain-cloud-services" => ServicePageContext {
            slug: "hosting-domain-cloud-services",
            title: "Hosting, Domain & Cloud Services",
            meta_title: "Hosting, Domain & Cloud Services | LKProfessionals",
            meta_description: "Hosting, domain, deployment, and cloud support services for secure, stable, and scalable digital operations.",
            canonical_path: "/services/hosting-domain-cloud-services",
            eyebrow: "Hosting, Domain & Cloud",
            hero_title: "Infrastructure support that keeps your digital presence stable, secure, and ready to scale.",
            hero_description: "We help businesses manage hosting, domains, deployment, and cloud-related needs so their websites and software products stay accessible, reliable, and better organized.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Companies that need dependable infrastructure guidance for websites, web apps, software platforms, or domain and deployment management.",
            hero_checklist: vec![
                "Hosting and domain setup or migration support",
                "Deployment readiness for sites and software",
                "Cloud planning for stability and growth",
            ],
            primary_cta_label: "Secure Your Infrastructure",
            primary_cta_href: "/contact",
            secondary_cta_label: "Explore Web Services",
            secondary_cta_href: "/services/web-development",
            stats: vec![
                ServiceStat {
                    value: "Reliable",
                    label: "Uptime-minded support",
                    note: "Stability and recoverability matter as much as the initial setup.",
                },
                ServiceStat {
                    value: "Organized",
                    label: "Cleaner ownership and access",
                    note: "We reduce confusion around domains, environments, and credentials.",
                },
                ServiceStat {
                    value: "Scalable",
                    label: "Ready for growth",
                    note: "Infrastructure choices consider future traffic, software expansion, and operational needs.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Messy hosting or domain ownership",
                    description: "We help untangle access, renewal, migration, and environment issues that create unnecessary risk.",
                },
                ServicePoint {
                    title: "Unclear deployment flow",
                    description: "We make launch and update paths more repeatable so releases are less fragile.",
                },
                ServicePoint {
                    title: "Infrastructure decisions made too late",
                    description: "We surface hosting and cloud planning earlier so the product foundation is more resilient.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Hosting and domain setup",
                    description: "Provisioning, DNS guidance, registrar coordination, and environment planning.",
                },
                ServiceDeliverable {
                    title: "Deployment support",
                    description: "Launch preparation for websites and applications, including environment alignment.",
                },
                ServiceDeliverable {
                    title: "Cloud and infrastructure guidance",
                    description: "Recommendations based on current needs, future scale, and operational simplicity.",
                },
                ServiceDeliverable {
                    title: "Migration and continuity help",
                    description: "Assistance when moving providers, updating environments, or reducing infrastructure risk.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Assess the current setup",
                    description: "We review what is hosted where, who owns what, and where the risks or bottlenecks live.",
                },
                ServiceStep {
                    title: "Plan the environment",
                    description: "Hosting, domain, deployment, and scaling decisions are shaped into a practical setup.",
                },
                ServiceStep {
                    title: "Implement or migrate",
                    description: "We execute setup, changes, or movement carefully to reduce disruption.",
                },
                ServiceStep {
                    title: "Stabilize and document",
                    description: "We help ensure the infrastructure is understandable, maintainable, and ready for future work.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Business continuity awareness",
                    description: "We treat infrastructure as an operational dependency, not just a technical checklist.",
                },
                ServicePoint {
                    title: "Bridges product and hosting concerns",
                    description: "Infrastructure planning reflects the websites, campaigns, and software it needs to support.",
                },
                ServicePoint {
                    title: "Practical clarity",
                    description: "We aim to leave clients with cleaner ownership, fewer surprises, and a more stable setup.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do you help with domain registration and transfers?",
                    answer: "Yes. We can assist with domain setup, transfer coordination, DNS guidance, and renewal clarity.",
                },
                ServiceFaq {
                    question: "Can you migrate our website or app to a new host?",
                    answer: "Yes. We can support migration planning and execution with an emphasis on minimizing disruption.",
                },
                ServiceFaq {
                    question: "Is this only for websites, or also for software platforms?",
                    answer: "Both. We support infrastructure needs for websites, web apps, dashboards, and larger software platforms.",
                },
            ],
            related_services: related_services(&[
                "web-development",
                "software-development",
                "seo-search-growth",
            ]),
        },
        "ai-automation-solutions" => ServicePageContext {
            slug: "ai-automation-solutions",
            title: "AI & Automation Solutions",
            meta_title: "AI Automation Solutions in Sri Lanka | LKProfessionals",
            meta_description: "AI and automation solutions for repetitive workflows, service operations, internal processes, and smarter business execution.",
            canonical_path: "/services/ai-automation-solutions",
            eyebrow: "AI & Automation Solutions",
            hero_title: "Automation that removes repetitive work and gives your team more room to think.",
            hero_description: "We help businesses identify processes that can be automated, augmented, or streamlined using practical automation design and AI-supported workflows.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Teams dealing with repetitive admin work, manual follow-ups, fragmented data handling, or customer workflows that should move faster with less human effort.",
            hero_checklist: vec![
                "Workflow automation and operational streamlining",
                "AI-assisted process support and task reduction",
                "Smarter routing, qualification, and internal handling",
            ],
            primary_cta_label: "Automate a Workflow",
            primary_cta_href: "/contact",
            secondary_cta_label: "See Custom Software",
            secondary_cta_href: "/services/custom-software-development",
            stats: vec![
                ServiceStat {
                    value: "Practical",
                    label: "Use-case driven",
                    note: "We target real process friction rather than forcing AI where it does not help.",
                },
                ServiceStat {
                    value: "Efficient",
                    label: "Time-saving focus",
                    note: "The goal is less repetitive work and faster movement through workflows.",
                },
                ServiceStat {
                    value: "Connected",
                    label: "Works with systems",
                    note: "Automation plans consider your forms, apps, dashboards, and team operations.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Teams wasting time on repetitive handling",
                    description: "We identify tasks that can be automated, assisted, or routed more intelligently.",
                },
                ServicePoint {
                    title: "Lead or service requests getting stuck",
                    description: "We improve intake, qualification, assignment, and follow-up flow so fewer opportunities go cold.",
                },
                ServicePoint {
                    title: "AI interest without a grounded use case",
                    description: "We focus on where automation creates practical operational value instead of trend-driven experimentation.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Workflow assessment",
                    description: "We identify process bottlenecks, handoff gaps, and repetitive tasks worth targeting first.",
                },
                ServiceDeliverable {
                    title: "Automation design",
                    description: "Rules, routing, triggers, and system touchpoints are mapped into a sensible automation flow.",
                },
                ServiceDeliverable {
                    title: "Implementation support",
                    description: "We help build or integrate the logic needed to run the automation reliably.",
                },
                ServiceDeliverable {
                    title: "Optimization after rollout",
                    description: "We monitor how the flow performs and improve weak spots or unnecessary complexity.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Spot the highest-friction workflow",
                    description: "We start where time loss, inconsistency, or delay is hurting the business most.",
                },
                ServiceStep {
                    title: "Design the automation path",
                    description: "Triggers, data movement, approvals, and outcomes are shaped into a clear workflow.",
                },
                ServiceStep {
                    title: "Implement and connect",
                    description: "The automation is integrated with forms, software, or internal systems as needed.",
                },
                ServiceStep {
                    title: "Tune for reliability",
                    description: "We refine edge cases and performance so the workflow becomes dependable in daily use.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Problem-first automation",
                    description: "We begin with business friction, not with tools looking for a reason to exist.",
                },
                ServicePoint {
                    title: "Operational empathy",
                    description: "Automation is planned around how staff, approvals, and exceptions really work.",
                },
                ServicePoint {
                    title: "Pairs well with custom systems",
                    description: "If the workflow needs software changes or dashboards, we can shape that path too.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "What kind of automation can you help with?",
                    answer: "We can support intake flows, routing, repetitive admin tasks, qualification logic, internal processes, and AI-assisted business workflows.",
                },
                ServiceFaq {
                    question: "Do we need a complex system already in place?",
                    answer: "Not always. Some automation can start with existing tools, while other use cases may benefit from custom software support.",
                },
                ServiceFaq {
                    question: "Can AI and automation be added to software you build?",
                    answer: "Yes. This service often works well alongside custom software, dashboards, and mobile or web platforms.",
                },
            ],
            related_services: related_services(&[
                "custom-software-development",
                "software-development",
                "it-consultation-digital-transformation",
            ]),
        },
        "it-consultation-digital-transformation" => ServicePageContext {
            slug: "it-consultation-digital-transformation",
            title: "IT Consultation & Digital Transformation",
            meta_title: "IT Consultation & Digital Transformation | LKProfessionals",
            meta_description: "IT consultation and digital transformation support for businesses planning technology upgrades, workflow improvements, and smarter systems.",
            canonical_path: "/services/it-consultation-digital-transformation",
            eyebrow: "IT Consultation & Digital Transformation",
            hero_title: "Technology decisions made clearer, smarter, and more commercially grounded.",
            hero_description: "We advise businesses on how to improve systems, modernize processes, choose the right digital direction, and avoid expensive technology missteps.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Leaders who know change is needed but want sharper decisions on priorities, systems, investments, and the best way to move into digital maturity.",
            hero_checklist: vec![
                "Technology roadmaps and digital planning",
                "System improvement and transformation strategy",
                "Guidance before major software or platform decisions",
            ],
            primary_cta_label: "Book a Consultation",
            primary_cta_href: "/contact",
            secondary_cta_label: "View All Services",
            secondary_cta_href: "/services",
            stats: vec![
                ServiceStat {
                    value: "Strategic",
                    label: "Decision support",
                    note: "We help teams focus on the right priorities instead of reacting to noise.",
                },
                ServiceStat {
                    value: "Grounded",
                    label: "Operational reality aware",
                    note: "Advice considers staffing, process maturity, and the pace of change your team can absorb.",
                },
                ServiceStat {
                    value: "Actionable",
                    label: "Clear next steps",
                    note: "Recommendations are meant to guide execution, not sit in a document.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Unsure what to digitize first",
                    description: "We help identify the highest-value process, platform, or system opportunities.",
                },
                ServicePoint {
                    title: "Technology spending without a roadmap",
                    description: "We bring structure to decisions so investments support an intentional transformation path.",
                },
                ServicePoint {
                    title: "Too much complexity around tools and systems",
                    description: "We simplify the picture and recommend a more coherent operating model.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Current-state assessment",
                    description: "A review of workflows, systems, bottlenecks, and digital maturity constraints.",
                },
                ServiceDeliverable {
                    title: "Transformation roadmap",
                    description: "Prioritized recommendations covering systems, sequencing, and practical next moves.",
                },
                ServiceDeliverable {
                    title: "Implementation direction",
                    description: "Advice on where software, websites, automation, or infrastructure changes should fit.",
                },
                ServiceDeliverable {
                    title: "Decision support for leadership",
                    description: "Clear framing that helps business leaders move forward with more confidence.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Understand the business reality",
                    description: "We gather how work currently happens, where frustration exists, and what goals matter most.",
                },
                ServiceStep {
                    title: "Identify transformation priorities",
                    description: "We isolate the changes most likely to create meaningful operational or commercial improvement.",
                },
                ServiceStep {
                    title: "Build the roadmap",
                    description: "Phasing, budget logic, dependencies, and service recommendations are shaped into a useful action plan.",
                },
                ServiceStep {
                    title: "Support execution choices",
                    description: "We help the team move from strategy into concrete implementation decisions.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Advice that leads somewhere",
                    description: "We focus on decisions, sequencing, and action, not abstract innovation language.",
                },
                ServicePoint {
                    title: "Cross-service perspective",
                    description: "Recommendations can span websites, software, automation, marketing, hosting, and process improvement.",
                },
                ServicePoint {
                    title: "Transformation with empathy",
                    description: "We account for the human and operational side of change, not just the tool stack.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Is this service only for large companies?",
                    answer: "No. Small and mid-sized businesses often benefit the most from clear digital planning before making bigger technology moves.",
                },
                ServiceFaq {
                    question: "Can consultation lead into actual implementation?",
                    answer: "Yes. We can help define the path and also support execution through our other service lines.",
                },
                ServiceFaq {
                    question: "Do you help review existing systems and vendors?",
                    answer: "Yes. We can assess the current setup, identify gaps, and recommend more effective next steps.",
                },
            ],
            related_services: related_services(&[
                "custom-software-development",
                "ai-automation-solutions",
                "hosting-domain-cloud-services",
            ]),
        },
        _ => service_page_context("web-development"),
    }
}

pub fn all_service_cards() -> Vec<ServiceCard> {
    vec![
        ServiceCard {
            slug: "/services/web-development",
            title: "Web Development",
            short_title: "Web Development",
            category: "Digital Presence",
            summary: "High-performance websites, landing pages, and web platforms built for trust, speed, and conversion.",
            icon: "fa-solid fa-globe",
        },
        ServiceCard {
            slug: "/services/mobile-app-development",
            title: "Mobile App Development",
            short_title: "Mobile Apps",
            category: "Product Delivery",
            summary: "User-friendly mobile experiences for customer engagement, internal workflows, and product growth.",
            icon: "fa-solid fa-mobile-screen-button",
        },
        ServiceCard {
            slug: "/services/custom-software-development",
            title: "Custom Software Development",
            short_title: "Custom Software",
            category: "Operations",
            summary: "Tailor-made systems for organizations that need software aligned to their exact business processes.",
            icon: "fa-solid fa-gears",
        },
        ServiceCard {
            slug: "/services/software-development",
            title: "Software Development",
            short_title: "Software Platforms",
            category: "Engineering",
            summary: "Scalable platforms, dashboards, and software products engineered for maintainability and control.",
            icon: "fa-solid fa-laptop-code",
        },
        ServiceCard {
            slug: "/services/digital-marketing",
            title: "Digital Marketing",
            short_title: "Digital Marketing",
            category: "Growth",
            summary: "Campaign strategy, content direction, and demand-generation support to increase online momentum.",
            icon: "fa-solid fa-bullhorn",
        },
        ServiceCard {
            slug: "/services/seo-search-growth",
            title: "SEO & Search Growth",
            short_title: "SEO",
            category: "Visibility",
            summary: "Technical SEO, content planning, and organic search optimization for long-term discoverability.",
            icon: "fa-solid fa-chart-line",
        },
        ServiceCard {
            slug: "/services/hosting-domain-cloud-services",
            title: "Hosting, Domain & Cloud Services",
            short_title: "Hosting & Cloud",
            category: "Infrastructure",
            summary: "Hosting, domain, deployment, and cloud guidance that keeps digital systems stable and organized.",
            icon: "fa-solid fa-cloud",
        },
        ServiceCard {
            slug: "/services/ai-automation-solutions",
            title: "AI & Automation Solutions",
            short_title: "AI & Automation",
            category: "Efficiency",
            summary: "Automation workflows and AI-assisted operations that reduce repetitive work and speed up handling.",
            icon: "fa-solid fa-robot",
        },
        ServiceCard {
            slug: "/services/it-consultation-digital-transformation",
            title: "IT Consultation & Digital Transformation",
            short_title: "IT Consultation",
            category: "Strategy",
            summary: "Technology guidance, roadmap planning, and digital transformation support for smarter decisions.",
            icon: "fa-solid fa-compass-drafting",
        },
    ]
}

fn related_services(slugs: &[&str]) -> Vec<ServiceCard> {
    all_service_cards()
        .into_iter()
        .filter(|service| {
            let route_slug = service.slug.trim_start_matches("/services/");
            slugs.contains(&route_slug)
        })
        .collect()
}
