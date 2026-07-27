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

#[derive(Clone, Copy)]
pub struct ServiceLink {
    pub title: &'static str,
    pub href: &'static str,
    pub description: &'static str,
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
    pub industries_served: Vec<&'static str>,
    pub technologies_used: Vec<&'static str>,
    pub related_solutions: Vec<ServiceLink>,
    pub related_case_studies: Vec<ServiceLink>,
    pub related_insights: Vec<ServiceLink>,
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
            meta_title: "Web Development Company | LKProfessionals",
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
            industries_served: industries_for("web-development"),
            technologies_used: technologies_for("web-development"),
            related_solutions: solution_links_for("web-development"),
            related_case_studies: case_study_links_for("web-development"),
            related_insights: insight_links_for("web-development"),
        },
        "mobile-app-development" => ServicePageContext {
            slug: "mobile-app-development",
            title: "Mobile App Development",
            meta_title: "Mobile App Development Company | LKProfessionals",
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
            industries_served: industries_for("mobile-app-development"),
            technologies_used: technologies_for("mobile-app-development"),
            related_solutions: solution_links_for("mobile-app-development"),
            related_case_studies: case_study_links_for("mobile-app-development"),
            related_insights: insight_links_for("mobile-app-development"),
        },
        "custom-software-development" => ServicePageContext {
            slug: "custom-software-development",
            title: "Custom Software Development",
            meta_title: "Custom Software Development Company | LKProfessionals",
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
            industries_served: industries_for("custom-software-development"),
            technologies_used: technologies_for("custom-software-development"),
            related_solutions: solution_links_for("custom-software-development"),
            related_case_studies: case_study_links_for("custom-software-development"),
            related_insights: insight_links_for("custom-software-development"),
        },
        "software-development" => ServicePageContext {
            slug: "software-development",
            title: "Software Development",
            meta_title: "Software Development Company | LKProfessionals",
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
            industries_served: industries_for("software-development"),
            technologies_used: technologies_for("software-development"),
            related_solutions: solution_links_for("software-development"),
            related_case_studies: case_study_links_for("software-development"),
            related_insights: insight_links_for("software-development"),
        },
        "digital-marketing" => ServicePageContext {
            slug: "digital-marketing",
            title: "Digital Marketing",
            meta_title: "Digital Marketing Services | LKProfessionals",
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
            industries_served: industries_for("digital-marketing"),
            technologies_used: technologies_for("digital-marketing"),
            related_solutions: solution_links_for("digital-marketing"),
            related_case_studies: case_study_links_for("digital-marketing"),
            related_insights: insight_links_for("digital-marketing"),
        },
        "seo-search-growth" => ServicePageContext {
            slug: "seo-search-growth",
            title: "SEO & Search Growth",
            meta_title: "SEO, GEO & Search Growth Services | LKProfessionals",
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
            industries_served: industries_for("seo-search-growth"),
            technologies_used: technologies_for("seo-search-growth"),
            related_solutions: solution_links_for("seo-search-growth"),
            related_case_studies: case_study_links_for("seo-search-growth"),
            related_insights: insight_links_for("seo-search-growth"),
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
            industries_served: industries_for("hosting-domain-cloud-services"),
            technologies_used: technologies_for("hosting-domain-cloud-services"),
            related_solutions: solution_links_for("hosting-domain-cloud-services"),
            related_case_studies: case_study_links_for("hosting-domain-cloud-services"),
            related_insights: insight_links_for("hosting-domain-cloud-services"),
        },
        "ai-automation-solutions" => ServicePageContext {
            slug: "ai-automation-solutions",
            title: "AI & Automation Solutions",
            meta_title: "AI Automation Solutions | LKProfessionals",
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
            industries_served: industries_for("ai-automation-solutions"),
            technologies_used: technologies_for("ai-automation-solutions"),
            related_solutions: solution_links_for("ai-automation-solutions"),
            related_case_studies: case_study_links_for("ai-automation-solutions"),
            related_insights: insight_links_for("ai-automation-solutions"),
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
            industries_served: industries_for("it-consultation-digital-transformation"),
            technologies_used: technologies_for("it-consultation-digital-transformation"),
            related_solutions: solution_links_for("it-consultation-digital-transformation"),
            related_case_studies: case_study_links_for("it-consultation-digital-transformation"),
            related_insights: insight_links_for("it-consultation-digital-transformation"),
        },
        "google-ads-agency" => ServicePageContext {
            slug: "google-ads-agency",
            title: "Google Ads Agency",
            meta_title: "Google Ads Agency | LKProfessionals",
            meta_description: "Google Ads management for service businesses that need stronger enquiry quality, better landing pages, and more disciplined PPC execution.",
            canonical_path: "/services/google-ads-agency",
            eyebrow: "Google Ads Agency",
            hero_title: "Google Ads campaigns built to generate qualified enquiries, not wasted clicks.",
            hero_description: "We plan, launch, and improve Google Ads campaigns for businesses that want commercial search visibility, tighter lead quality, and landing pages that support conversion instead of leaking budget.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Businesses that need fast demand generation, better PPC structure, or a paid-search partner who can connect campaign intent with website performance.",
            hero_checklist: vec![
                "Search campaigns for high-intent service keywords",
                "Landing page alignment for stronger conversion rates",
                "Ongoing optimization around lead quality and cost control",
            ],
            primary_cta_label: "Plan Your Google Ads Campaign",
            primary_cta_href: "/request-quote",
            secondary_cta_label: "Talk to LKProfessionals",
            secondary_cta_href: "/contact",
            stats: vec![
                ServiceStat {
                    value: "Intent-led",
                    label: "Search-first targeting",
                    note: "Campaigns focus on the terms most likely to produce commercial action.",
                },
                ServiceStat {
                    value: "Sharper",
                    label: "Lead quality control",
                    note: "Keyword, ad, and landing page decisions are shaped to reduce low-fit enquiries.",
                },
                ServiceStat {
                    value: "Measured",
                    label: "Optimization discipline",
                    note: "We use search-term, conversion, and page-quality signals to improve performance over time.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Budget disappearing into broad traffic",
                    description: "We tighten campaign structure and search intent targeting so spend supports realistic buying behavior.",
                },
                ServicePoint {
                    title: "Clicks arriving on weak landing pages",
                    description: "We align ads with clearer pages, stronger messaging, and more deliberate CTAs.",
                },
                ServicePoint {
                    title: "No clear view of what is working",
                    description: "We create a cleaner optimization loop around conversion paths, search terms, and campaign priorities.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Campaign architecture and keyword planning",
                    description: "Ad groups, keyword intent, and conversion priorities structured around the offer.",
                },
                ServiceDeliverable {
                    title: "Ad copy and extension setup",
                    description: "Message testing shaped to match user intent and service credibility.",
                },
                ServiceDeliverable {
                    title: "Landing page coordination",
                    description: "Recommendations or implementation support for the pages responsible for converting paid traffic.",
                },
                ServiceDeliverable {
                    title: "Management and optimization",
                    description: "Ongoing reviews of search terms, bids, conversion performance, and page fit.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Clarify commercial intent",
                    description: "We define which services, locations, and buyer actions the campaign should target first.",
                },
                ServiceStep {
                    title: "Build the campaign structure",
                    description: "Keyword clusters, ad messaging, exclusions, and conversion actions are organized deliberately.",
                },
                ServiceStep {
                    title: "Launch with landing-page support",
                    description: "Campaigns go live alongside the page and CTA conditions needed to convert traffic cleanly.",
                },
                ServiceStep {
                    title: "Optimize around signal quality",
                    description: "We refine based on search intent, cost efficiency, and the quality of resulting enquiries.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Paid search tied to business reality",
                    description: "Campaign strategy reflects your service model, buyer cycle, and landing page strength.",
                },
                ServicePoint {
                    title: "Landing experience matters",
                    description: "We do not treat ads and web pages as separate systems when conversion depends on both.",
                },
                ServicePoint {
                    title: "Useful accountability",
                    description: "Optimization is driven by commercial signal quality, not vanity metrics alone.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do you only manage Google Search Ads?",
                    answer: "Search campaigns are often the highest-priority starting point for service businesses, but we can also advise on wider campaign structure where appropriate.",
                },
                ServiceFaq {
                    question: "Can you improve the landing page as well?",
                    answer: "Yes. We frequently support landing page structure, copy direction, and CTA clarity because campaign performance depends on the page after the click.",
                },
                ServiceFaq {
                    question: "Is Google Ads useful if we are already investing in SEO?",
                    answer: "Yes. Paid search can capture immediate demand while SEO compounds longer-term visibility, especially when both target the same commercial intent carefully.",
                },
            ],
            related_services: related_services(&[
                "digital-marketing",
                "seo-search-growth",
                "web-development",
            ]),
            industries_served: industries_for("google-ads-agency"),
            technologies_used: technologies_for("google-ads-agency"),
            related_solutions: solution_links_for("google-ads-agency"),
            related_case_studies: case_study_links_for("google-ads-agency"),
            related_insights: insight_links_for("google-ads-agency"),
        },
        "local-seo-services" => ServicePageContext {
            slug: "local-seo-services",
            title: "Local SEO Services",
            meta_title: "Local SEO Services in Sri Lanka | LKProfessionals",
            meta_description: "Local SEO services for Sri Lankan businesses that need stronger map visibility, service-area relevance, and more qualified local enquiries.",
            canonical_path: "/services/local-seo-services",
            eyebrow: "Local SEO Services",
            hero_title: "Local search visibility that helps nearby customers find the right service faster.",
            hero_description: "We improve local SEO for businesses that depend on city-based visibility, service-area relevance, and stronger trust signals across their website, Google Business presence, and supporting location content.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Service businesses, clinics, agencies, and multi-location brands that want better local discovery in Sri Lanka without relying only on paid traffic.",
            hero_checklist: vec![
                "Google Business and location signal optimization",
                "Service-area and city-page support",
                "On-page and internal linking improvements for local intent",
            ],
            primary_cta_label: "Strengthen Local Visibility",
            primary_cta_href: "/request-quote",
            secondary_cta_label: "Explore SEO Services",
            secondary_cta_href: "/services/seo-search-growth",
            stats: vec![
                ServiceStat {
                    value: "Local",
                    label: "Area relevance focus",
                    note: "We align pages and signals with where your business actually serves customers.",
                },
                ServiceStat {
                    value: "Trust-led",
                    label: "Business presence clarity",
                    note: "Address, service scope, entity consistency, and page quality work together.",
                },
                ServiceStat {
                    value: "Scalable",
                    label: "Supports expansion",
                    note: "The structure can grow from one location to multiple target areas without becoming thin.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Ranking weakly in the areas that matter most",
                    description: "We improve page intent, location relevance, and supporting internal links so local signals become clearer.",
                },
                ServicePoint {
                    title: "Service pages written too broadly",
                    description: "We help connect commercial service pages with the local contexts buyers actually search from.",
                },
                ServicePoint {
                    title: "Google Business presence not fully supporting the website",
                    description: "We align the website structure with the visibility work happening around your local business profile.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Local SEO audit and opportunity map",
                    description: "A practical review of location targeting, page quality, internal linking, and local search signals.",
                },
                ServiceDeliverable {
                    title: "Local page and service alignment",
                    description: "Recommendations or implementation support for location-aware service pages and nearby search intent.",
                },
                ServiceDeliverable {
                    title: "Google Business and citation guidance",
                    description: "Advice on business profile alignment and supporting trust signals where relevant.",
                },
                ServiceDeliverable {
                    title: "Ongoing optimization priorities",
                    description: "A staged plan for expanding local visibility without creating duplicate or weak pages.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Assess current local visibility",
                    description: "We review what areas you target, how local intent is handled on-site, and where gaps exist.",
                },
                ServiceStep {
                    title: "Strengthen location relevance",
                    description: "We improve page focus, internal linking, and service-to-location alignment.",
                },
                ServiceStep {
                    title: "Support trust and discoverability",
                    description: "Entity consistency, business profile considerations, and content support are addressed where useful.",
                },
                ServiceStep {
                    title: "Expand deliberately",
                    description: "Once the foundation works, we help prioritize the next locations or service-area opportunities.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Local SEO without thin-page sprawl",
                    description: "We focus on useful, commercially aligned location relevance instead of mass-producing weak pages.",
                },
                ServicePoint {
                    title: "Commercial service pages come first",
                    description: "Local visibility works better when the core service pages are strong enough to deserve the traffic.",
                },
                ServicePoint {
                    title: "Built for Sri Lankan search realities",
                    description: "Recommendations consider how local service buyers search across cities, districts, and nearby alternatives.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do we need a separate page for every city?",
                    answer: "Not always. The right structure depends on your service model, proof of local relevance, and whether each page can offer distinct value.",
                },
                ServiceFaq {
                    question: "Can local SEO help a business outside Colombo?",
                    answer: "Yes. Local SEO matters anywhere buyers search for nearby or area-specific services, including Jaffna and other regional markets.",
                },
                ServiceFaq {
                    question: "Is this different from broader SEO?",
                    answer: "Yes. Local SEO focuses more heavily on location intent, service-area relevance, business presence signals, and nearby discoverability.",
                },
            ],
            related_services: related_services(&[
                "seo-search-growth",
                "web-development",
                "digital-marketing",
            ]),
            industries_served: industries_for("local-seo-services"),
            technologies_used: technologies_for("local-seo-services"),
            related_solutions: solution_links_for("local-seo-services"),
            related_case_studies: case_study_links_for("local-seo-services"),
            related_insights: insight_links_for("local-seo-services"),
        },
        "website-maintenance-services" => ServicePageContext {
            slug: "website-maintenance-services",
            title: "Website Maintenance Services",
            meta_title: "Website Maintenance Services | LKProfessionals",
            meta_description: "Website maintenance services covering updates, fixes, monitoring, backups, performance checks, and support for business-critical websites.",
            canonical_path: "/services/website-maintenance-services",
            eyebrow: "Website Maintenance Services",
            hero_title: "Website maintenance that protects performance, trust, and day-to-day business continuity.",
            hero_description: "We support business websites after launch with structured maintenance, technical fixes, monitoring, backups, and practical support so your site stays reliable instead of quietly degrading.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Teams that rely on their website for enquiries, campaigns, visibility, or customer trust and need dependable support beyond the initial launch.",
            hero_checklist: vec![
                "Ongoing technical support and updates",
                "Monitoring, backup, and recovery discipline",
                "Performance, SEO, and security-minded maintenance",
            ],
            primary_cta_label: "Get Website Support",
            primary_cta_href: "/request-quote",
            secondary_cta_label: "Talk to LKProfessionals",
            secondary_cta_href: "/contact",
            stats: vec![
                ServiceStat {
                    value: "Protected",
                    label: "Stability-first care",
                    note: "We reduce the risk of avoidable breakage, downtime, and silent technical decay.",
                },
                ServiceStat {
                    value: "Responsive",
                    label: "Issue-handling support",
                    note: "Fixes and updates move through a clearer process than ad hoc emergency requests.",
                },
                ServiceStat {
                    value: "Useful",
                    label: "Business-impact aware",
                    note: "Maintenance work considers SEO, conversion flow, and user experience, not just code changes.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "No one actively maintaining the website",
                    description: "We create a structured support layer so small issues do not turn into larger business problems.",
                },
                ServicePoint {
                    title: "Content, design, or integrations falling out of date",
                    description: "We keep the site usable and commercially credible as the business changes.",
                },
                ServicePoint {
                    title: "Fear of updates breaking something important",
                    description: "We handle maintenance with a steadier process that reduces avoidable release risk.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Routine maintenance and updates",
                    description: "Planned changes, bug fixes, content support, and technical housekeeping.",
                },
                ServiceDeliverable {
                    title: "Monitoring and backup oversight",
                    description: "A more reliable posture around uptime awareness, backups, and recovery readiness.",
                },
                ServiceDeliverable {
                    title: "Performance and SEO checks",
                    description: "Basic maintenance work informed by how site quality affects search visibility and conversion.",
                },
                ServiceDeliverable {
                    title: "Support for growth changes",
                    description: "Help with adding sections, adjusting CTAs, improving pages, or supporting new campaigns.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Review the current website condition",
                    description: "We assess the setup, known issues, dependencies, and the level of support the site really needs.",
                },
                ServiceStep {
                    title: "Stabilize the essentials",
                    description: "Urgent fixes, update priorities, and risk areas are handled before longer-term improvements.",
                },
                ServiceStep {
                    title: "Move into structured maintenance",
                    description: "Requests, updates, and preventative checks follow a more predictable support rhythm.",
                },
                ServiceStep {
                    title: "Improve over time",
                    description: "Maintenance becomes a platform for incremental UX, SEO, and conversion improvements as needed.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Support with commercial awareness",
                    description: "We understand that a business website needs to stay credible, discoverable, and conversion-ready.",
                },
                ServicePoint {
                    title: "Not limited to emergency fixes",
                    description: "We can also support page improvements, content structure, and technical refinements that keep the site stronger over time.",
                },
                ServicePoint {
                    title: "Useful bridge between launch and growth",
                    description: "Maintenance becomes the layer that keeps the website usable while bigger improvements are phased in.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do you only maintain websites you originally built?",
                    answer: "No. We can often take over support for an existing website after reviewing the platform, risks, and practical maintenance needs.",
                },
                ServiceFaq {
                    question: "Can maintenance include small SEO or conversion improvements?",
                    answer: "Yes. Where appropriate, maintenance work can include better page structure, CTA refinements, and technical fixes that support search visibility.",
                },
                ServiceFaq {
                    question: "Is hosting included?",
                    answer: "Maintenance and hosting are related but distinct. We can support both and recommend the right mix based on how the site is currently managed.",
                },
            ],
            related_services: related_services(&[
                "hosting-domain-cloud-services",
                "web-development",
                "seo-search-growth",
            ]),
            industries_served: industries_for("website-maintenance-services"),
            technologies_used: technologies_for("website-maintenance-services"),
            related_solutions: solution_links_for("website-maintenance-services"),
            related_case_studies: case_study_links_for("website-maintenance-services"),
            related_insights: insight_links_for("website-maintenance-services"),
        },
        "website-redesign-services" => ServicePageContext {
            slug: "website-redesign-services",
            title: "Website Redesign Services",
            meta_title: "Website Redesign Services | LKProfessionals",
            meta_description: "Website redesign services for businesses that need stronger trust, better conversion flow, improved SEO structure, and a more modern digital presence.",
            canonical_path: "/services/website-redesign-services",
            eyebrow: "Website Redesign Services",
            hero_title: "Website redesigns that improve trust, clarity, and commercial performance without losing direction.",
            hero_description: "We redesign business websites that have become outdated, difficult to manage, conversion-weak, or disconnected from the company they are supposed to represent today.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Businesses with aging websites, unclear service presentation, weak enquiry flow, or a digital presence that no longer matches their standards.",
            hero_checklist: vec![
                "Modern design and content restructuring",
                "SEO-aware redesign planning",
                "Conversion-path improvements across key pages",
            ],
            primary_cta_label: "Discuss a Website Redesign",
            primary_cta_href: "/request-quote",
            secondary_cta_label: "See Web Development",
            secondary_cta_href: "/services/web-development",
            stats: vec![
                ServiceStat {
                    value: "Clearer",
                    label: "Better message hierarchy",
                    note: "We simplify how the website explains the business and guides the visitor.",
                },
                ServiceStat {
                    value: "Stronger",
                    label: "Trust and conversion focus",
                    note: "Design decisions are tied to credibility, usability, and enquiry quality.",
                },
                ServiceStat {
                    value: "Safer",
                    label: "SEO-aware transition",
                    note: "Redesign planning considers rankings, internal links, and page continuity before launch.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "The website no longer reflects the business properly",
                    description: "We reposition the site so the offer, quality, and priorities are represented more convincingly.",
                },
                ServicePoint {
                    title: "Users reach the site but do not take action",
                    description: "We improve message flow, CTA placement, and page structure to reduce conversion friction.",
                },
                ServicePoint {
                    title: "Redesign risk around SEO and existing traffic",
                    description: "We approach structural change carefully so visibility is not damaged by unnecessary disruption.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Website review and redesign roadmap",
                    description: "A clear view of what should change, what should be preserved, and how the site should improve.",
                },
                ServiceDeliverable {
                    title: "Page architecture and content restructuring",
                    description: "Sharper hierarchy for service pages, trust signals, and conversion paths.",
                },
                ServiceDeliverable {
                    title: "Responsive redesign implementation",
                    description: "A more current visual system and front-end experience that still respects usability and performance.",
                },
                ServiceDeliverable {
                    title: "Launch transition support",
                    description: "Redirect awareness, QA, and structural checks to protect the move into the new version.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Audit the current website",
                    description: "We identify what is weakening trust, search visibility, usability, or conversion performance.",
                },
                ServiceStep {
                    title: "Restructure the key journeys",
                    description: "The homepage, service pages, and CTA flow are reorganized around business priorities.",
                },
                ServiceStep {
                    title: "Design and rebuild deliberately",
                    description: "We implement the redesign with attention to speed, responsiveness, and continuity.",
                },
                ServiceStep {
                    title: "Launch with reduced risk",
                    description: "Technical checks, redirects, and post-launch review help the redesign land more cleanly.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Redesign for outcomes, not just aesthetics",
                    description: "We treat visual improvement as one part of a broader credibility and conversion upgrade.",
                },
                ServicePoint {
                    title: "SEO and UX considered together",
                    description: "Page structure, metadata, internal links, and message hierarchy are part of the same redesign conversation.",
                },
                ServicePoint {
                    title: "Grounded modernization",
                    description: "The result aims to feel more capable and current without turning the site into trend-driven noise.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Do we need a full rebuild to redesign the site?",
                    answer: "Not always. Some websites need a full rebuild, while others can be improved through restructuring and selective redevelopment.",
                },
                ServiceFaq {
                    question: "Can a redesign help with SEO?",
                    answer: "Yes, if handled properly. A redesign can improve structure, internal linking, performance, and page quality, but it needs careful planning to avoid regressions.",
                },
                ServiceFaq {
                    question: "Can you redesign only a few key pages first?",
                    answer: "Yes. In some cases a phased redesign focused on the homepage and core service pages is the most sensible starting point.",
                },
            ],
            related_services: related_services(&[
                "web-development",
                "seo-search-growth",
                "website-maintenance-services",
            ]),
            industries_served: industries_for("website-redesign-services"),
            technologies_used: technologies_for("website-redesign-services"),
            related_solutions: solution_links_for("website-redesign-services"),
            related_case_studies: case_study_links_for("website-redesign-services"),
            related_insights: insight_links_for("website-redesign-services"),
        },
        "crm-software-development" => ServicePageContext {
            slug: "crm-software-development",
            title: "CRM Software Development",
            meta_title: "CRM Software Development Company | LKProfessionals",
            meta_description: "CRM software development for sales pipelines, customer follow-up, lead assignment, reporting visibility, and relationship-driven operations.",
            canonical_path: "/services/crm-software-development",
            eyebrow: "CRM Software Development",
            hero_title: "Custom CRM software that gives your team clearer pipelines, cleaner follow-up, and better customer visibility.",
            hero_description: "We build CRM systems for businesses that have outgrown spreadsheets, disconnected sales tools, or generic platforms that do not reflect how their customer relationships are actually managed.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Sales teams, service businesses, and growing organizations that need tighter lead tracking, customer history, and operational visibility.",
            hero_checklist: vec![
                "Lead, customer, and opportunity management",
                "Pipeline stages, tasks, follow-up, and team accountability",
                "Reports and dashboards for sales or service visibility",
            ],
            primary_cta_label: "Plan Your CRM System",
            primary_cta_href: "/request-quote",
            secondary_cta_label: "Talk to LKProfessionals",
            secondary_cta_href: "/contact",
            stats: vec![
                ServiceStat {
                    value: "Visible",
                    label: "Pipeline clarity",
                    note: "The right CRM gives managers and teams a shared view of customer progress.",
                },
                ServiceStat {
                    value: "Structured",
                    label: "Follow-up discipline",
                    note: "Tasks, reminders, ownership, and stage logic reduce opportunities slipping through gaps.",
                },
                ServiceStat {
                    value: "Tailored",
                    label: "Business-fit design",
                    note: "We shape CRM flows around your sales and service model instead of forcing generic behavior.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Lead and customer information scattered everywhere",
                    description: "We centralize records, activity history, and ownership so the team works from one clearer system.",
                },
                ServicePoint {
                    title: "Inconsistent follow-up and lost opportunities",
                    description: "We build task flow and pipeline discipline into the product so accountability improves.",
                },
                ServicePoint {
                    title: "Reports that do not reflect operational reality",
                    description: "We structure dashboards and views around the decisions your business actually needs to make.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "CRM discovery and workflow mapping",
                    description: "We model how leads, customers, tasks, and handoffs move through your operation.",
                },
                ServiceDeliverable {
                    title: "Pipeline, contact, and activity design",
                    description: "Stages, records, reminders, notes, and ownership rules are shaped into a practical system.",
                },
                ServiceDeliverable {
                    title: "Reporting and managerial visibility",
                    description: "Dashboards and summaries that show the state of sales or service performance more clearly.",
                },
                ServiceDeliverable {
                    title: "Rollout and refinement support",
                    description: "We help the first release become usable in real team workflows, then improve based on adoption feedback.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Understand the relationship workflow",
                    description: "We identify how enquiries, customers, follow-up, and internal accountability currently work.",
                },
                ServiceStep {
                    title: "Design the CRM around real behavior",
                    description: "Roles, stages, tasks, notes, and reports are structured to fit the operation.",
                },
                ServiceStep {
                    title: "Build the core modules",
                    description: "We implement the records, pipeline flows, dashboards, and controls the team needs first.",
                },
                ServiceStep {
                    title: "Support adoption and iteration",
                    description: "The system is refined based on real usage, not only assumptions made before launch.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "CRM built around your business logic",
                    description: "We care about the real sales and service flow, not just generic contact storage.",
                },
                ServicePoint {
                    title: "Commercial workflow awareness",
                    description: "The goal is clearer follow-up, better visibility, and stronger relationship handling across the team.",
                },
                ServicePoint {
                    title: "Natural fit with broader software delivery",
                    description: "If the CRM needs portals, booking flows, automation, or integrations, we can shape that wider system too.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Why build a custom CRM instead of using a standard tool?",
                    answer: "A custom CRM makes sense when your process, reporting needs, or team workflow are too specific for a generic platform to handle well.",
                },
                ServiceFaq {
                    question: "Can a CRM include service follow-up as well as sales?",
                    answer: "Yes. CRM systems can be designed for sales pipelines, customer servicing, retention workflows, or a blend of all three.",
                },
                ServiceFaq {
                    question: "Can you build a CRM in phases?",
                    answer: "Yes. Many teams start with contacts, pipelines, and reporting, then add automation, integrations, or additional modules later.",
                },
            ],
            related_services: related_services(&[
                "custom-software-development",
                "software-development",
                "ai-automation-solutions",
            ]),
            industries_served: industries_for("crm-software-development"),
            technologies_used: technologies_for("crm-software-development"),
            related_solutions: solution_links_for("crm-software-development"),
            related_case_studies: case_study_links_for("crm-software-development"),
            related_insights: insight_links_for("crm-software-development"),
        },
        "inventory-management-software" => ServicePageContext {
            slug: "inventory-management-software",
            title: "Inventory Management Software",
            meta_title: "Inventory Management Software Development | LKProfessionals",
            meta_description: "Inventory management software for stock visibility, purchasing control, warehouse accuracy, branch coordination, and decision-ready reporting.",
            canonical_path: "/services/inventory-management-software",
            eyebrow: "Inventory Management Software",
            hero_title: "Inventory systems that give growing businesses tighter stock control and fewer operational surprises.",
            hero_description: "We build inventory management software for businesses that need clearer stock visibility, better replenishment decisions, stronger branch coordination, and a more reliable view of what is actually happening across products and locations.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Retailers, distributors, service businesses with stocked items, and multi-branch operations that have outgrown spreadsheets or fragmented stock tools.",
            hero_checklist: vec![
                "Stock, purchase, and supplier workflow control",
                "Branch, warehouse, and movement visibility",
                "Reporting for replenishment and operational decisions",
            ],
            primary_cta_label: "Build an Inventory System",
            primary_cta_href: "/request-quote",
            secondary_cta_label: "Discuss Your Workflow",
            secondary_cta_href: "/contact",
            stats: vec![
                ServiceStat {
                    value: "Accurate",
                    label: "Better stock visibility",
                    note: "The right system reduces uncertainty around what is available, moving, or at risk.",
                },
                ServiceStat {
                    value: "Controlled",
                    label: "Purchasing discipline",
                    note: "Teams gain clearer signals for reordering, supplier handling, and stock movement.",
                },
                ServiceStat {
                    value: "Scalable",
                    label: "Supports branch growth",
                    note: "The structure can expand into warehouses, stores, and broader product complexity over time.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Stock counts and spreadsheets not matching reality",
                    description: "We help replace fragile manual tracking with a system that shows movement and availability more clearly.",
                },
                ServicePoint {
                    title: "Purchasing decisions made with poor visibility",
                    description: "We create reporting and stock logic that supports replenishment and supplier planning.",
                },
                ServicePoint {
                    title: "Multiple branches or stores operating without coordination",
                    description: "We design workflows that make stock movement and location-level visibility easier to manage.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Inventory workflow discovery",
                    description: "We map products, suppliers, purchases, adjustments, movements, and reporting needs.",
                },
                ServiceDeliverable {
                    title: "Stock and warehouse system design",
                    description: "Product records, movement logic, and role-based controls are structured around your operation.",
                },
                ServiceDeliverable {
                    title: "Reporting and visibility modules",
                    description: "Dashboards for stock status, purchasing patterns, and operational exceptions.",
                },
                ServiceDeliverable {
                    title: "Expansion-ready implementation",
                    description: "A foundation that can later connect with POS, e-commerce, barcode workflows, or broader ERP logic.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Map current stock handling",
                    description: "We identify where inaccuracy, delays, or blind spots are affecting the business most.",
                },
                ServiceStep {
                    title: "Design the control model",
                    description: "Products, locations, users, purchase flows, and reporting rules are shaped into the system.",
                },
                ServiceStep {
                    title: "Build the operational core",
                    description: "The high-value stock workflows are implemented first so teams can start improving visibility quickly.",
                },
                ServiceStep {
                    title: "Refine around real usage",
                    description: "Feedback from daily operations helps improve reports, controls, and branch-specific needs.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Inventory control with operational realism",
                    description: "We design for the exceptions, approvals, and day-to-day pressures that stock systems must survive.",
                },
                ServicePoint {
                    title: "Clear path into broader business systems",
                    description: "Inventory projects can evolve into POS, CRM, ERP, or e-commerce integrations when the business is ready.",
                },
                ServicePoint {
                    title: "Decision support built in",
                    description: "Reporting matters because inventory is not only about counting; it affects purchasing, cash flow, and service quality.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Is this only for retail companies?",
                    answer: "No. Inventory systems can also support distributors, pharmacies, workshops, service businesses with stocked items, and multi-location operations.",
                },
                ServiceFaq {
                    question: "Can inventory software connect with POS or e-commerce later?",
                    answer: "Yes. Many businesses start with inventory control and then connect it to sales, ordering, or reporting systems over time.",
                },
                ServiceFaq {
                    question: "Can you support barcode-based workflows?",
                    answer: "Yes. Barcode handling can be included where it improves receiving, stock movement, sales, or counting processes.",
                },
            ],
            related_services: related_services(&[
                "custom-software-development",
                "software-development",
                "booking-system-development",
            ]),
            industries_served: industries_for("inventory-management-software"),
            technologies_used: technologies_for("inventory-management-software"),
            related_solutions: solution_links_for("inventory-management-software"),
            related_case_studies: case_study_links_for("inventory-management-software"),
            related_insights: insight_links_for("inventory-management-software"),
        },
        "booking-system-development" => ServicePageContext {
            slug: "booking-system-development",
            title: "Booking System Development",
            meta_title: "Booking System Development Company | LKProfessionals",
            meta_description: "Booking system development for appointments, reservations, scheduling logic, confirmations, resource allocation, and service workflow control.",
            canonical_path: "/services/booking-system-development",
            eyebrow: "Booking System Development",
            hero_title: "Booking systems that make appointments, reservations, and scheduling easier to manage at scale.",
            hero_description: "We build booking systems for service businesses and institutions that need cleaner scheduling, fewer manual coordination issues, and a customer experience that makes booking feel straightforward.",
            hero_panel_title: "Best fit for",
            hero_panel_body: "Clinics, education providers, hospitality brands, consultancies, and service teams that need appointment, reservation, or schedule management built around their real workflow.",
            hero_checklist: vec![
                "Appointment, reservation, and resource scheduling",
                "Availability logic, confirmations, and staff coordination",
                "Admin dashboards and operational visibility",
            ],
            primary_cta_label: "Plan a Booking Platform",
            primary_cta_href: "/request-quote",
            secondary_cta_label: "Talk to LKProfessionals",
            secondary_cta_href: "/contact",
            stats: vec![
                ServiceStat {
                    value: "Simpler",
                    label: "Lower coordination friction",
                    note: "Users and staff both benefit when booking logic is easier to understand and act on.",
                },
                ServiceStat {
                    value: "Organized",
                    label: "Better schedule control",
                    note: "Availability, assignments, and changes are managed through a clearer system.",
                },
                ServiceStat {
                    value: "Scalable",
                    label: "Supports growing demand",
                    note: "The booking flow can expand across locations, staff, services, or customer types.",
                },
            ],
            challenges: vec![
                ServicePoint {
                    title: "Manual booking handling consuming too much staff time",
                    description: "We automate and structure the booking flow so teams spend less time coordinating basic availability.",
                },
                ServicePoint {
                    title: "Customer booking journeys that feel confusing",
                    description: "We simplify the front-end booking experience so customers can complete the right action with less friction.",
                },
                ServicePoint {
                    title: "Operational rules too specific for generic tools",
                    description: "We shape the system around actual service lengths, capacities, approvals, or assignment logic.",
                },
            ],
            deliverables: vec![
                ServiceDeliverable {
                    title: "Booking workflow discovery",
                    description: "We define how appointments, reservations, resources, and exceptions should really behave.",
                },
                ServiceDeliverable {
                    title: "Customer-facing booking experience",
                    description: "A front-end flow designed to reduce confusion and improve completion rates.",
                },
                ServiceDeliverable {
                    title: "Admin and operational controls",
                    description: "Dashboards, calendars, notifications, and rule handling for the internal team.",
                },
                ServiceDeliverable {
                    title: "Integration and expansion planning",
                    description: "The system can be shaped to connect with payments, CRM, reporting, or broader software tools.",
                },
            ],
            process: vec![
                ServiceStep {
                    title: "Understand the scheduling reality",
                    description: "We review how services, staff, resources, capacity, and customer actions currently interact.",
                },
                ServiceStep {
                    title: "Design the booking logic",
                    description: "Availability rules, confirmations, constraints, and admin handling are structured carefully.",
                },
                ServiceStep {
                    title: "Build the key flows",
                    description: "We implement the booking experience and the operational control layer needed to run it confidently.",
                },
                ServiceStep {
                    title: "Improve through live usage",
                    description: "Once the system is in use, we refine edge cases, reporting, and conversion friction based on real demand.",
                },
            ],
            differentiators: vec![
                ServicePoint {
                    title: "Booking systems built around operations",
                    description: "We consider staff coordination, approval logic, and resource constraints as core design inputs.",
                },
                ServicePoint {
                    title: "Customer experience matters too",
                    description: "The booking flow should help users complete the action, not only satisfy the backend logic.",
                },
                ServicePoint {
                    title: "Flexible enough for industry-specific needs",
                    description: "The same core approach can support clinics, training, hospitality, or service businesses with very different scheduling rules.",
                },
            ],
            faqs: vec![
                ServiceFaq {
                    question: "Can a booking system include payments or reminders?",
                    answer: "Yes. Depending on the workflow, the system can include confirmations, reminders, payments, or other customer-handling steps.",
                },
                ServiceFaq {
                    question: "Is this useful only for hotels?",
                    answer: "No. Booking systems are useful for clinics, educators, consultants, service providers, and many appointment-driven businesses.",
                },
                ServiceFaq {
                    question: "Can you integrate booking with CRM or custom software?",
                    answer: "Yes. Booking often works best when it connects with customer records, internal dashboards, or broader workflow systems.",
                },
            ],
            related_services: related_services(&[
                "custom-software-development",
                "mobile-app-development",
                "crm-software-development",
            ]),
            industries_served: industries_for("booking-system-development"),
            technologies_used: technologies_for("booking-system-development"),
            related_solutions: solution_links_for("booking-system-development"),
            related_case_studies: case_study_links_for("booking-system-development"),
            related_insights: insight_links_for("booking-system-development"),
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
            slug: "/services/google-ads-agency",
            title: "Google Ads Agency",
            short_title: "Google Ads",
            category: "Paid Growth",
            summary: "Google Ads strategy, campaign management, landing page alignment, and qualified lead generation support.",
            icon: "fa-solid fa-rectangle-ad",
        },
        ServiceCard {
            slug: "/services/local-seo-services",
            title: "Local SEO Services",
            short_title: "Local SEO",
            category: "Visibility",
            summary: "Local search optimization for Sri Lankan businesses that need stronger map visibility, service-area relevance, and enquiry quality.",
            icon: "fa-solid fa-map-location-dot",
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
            slug: "/services/website-maintenance-services",
            title: "Website Maintenance Services",
            short_title: "Website Maintenance",
            category: "Support",
            summary: "Ongoing updates, fixes, monitoring, backup discipline, and technical support for business-critical websites.",
            icon: "fa-solid fa-screwdriver-wrench",
        },
        ServiceCard {
            slug: "/services/website-redesign-services",
            title: "Website Redesign Services",
            short_title: "Website Redesign",
            category: "Digital Presence",
            summary: "Website redesign work for businesses that need stronger trust, better conversion flow, and a more current digital presence.",
            icon: "fa-solid fa-pen-ruler",
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
            slug: "/services/crm-software-development",
            title: "CRM Software Development",
            short_title: "CRM Development",
            category: "Operations",
            summary: "Custom CRM systems for sales pipelines, follow-up control, team visibility, and customer relationship workflows.",
            icon: "fa-solid fa-address-book",
        },
        ServiceCard {
            slug: "/services/inventory-management-software",
            title: "Inventory Management Software",
            short_title: "Inventory Systems",
            category: "Operations",
            summary: "Inventory control systems for stock visibility, purchasing accuracy, reporting clarity, and multi-location coordination.",
            icon: "fa-solid fa-boxes-stacked",
        },
        ServiceCard {
            slug: "/services/booking-system-development",
            title: "Booking System Development",
            short_title: "Booking Systems",
            category: "Operations",
            summary: "Booking systems for appointments, reservations, scheduling logic, confirmations, and customer-facing service workflows.",
            icon: "fa-solid fa-calendar-check",
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

fn service_link(title: &'static str, href: &'static str, description: &'static str) -> ServiceLink {
    ServiceLink {
        title,
        href,
        description,
    }
}

fn industries_for(slug: &str) -> Vec<&'static str> {
    match slug {
        "web-development" | "website-redesign-services" | "website-maintenance-services" => vec![
            "Professional services firms",
            "Healthcare and clinics",
            "Education and training providers",
            "Retail and e-commerce brands",
            "Hospitality and service businesses",
        ],
        "seo-search-growth" | "local-seo-services" | "digital-marketing" | "google-ads-agency" => {
            vec![
                "Local service businesses",
                "Professional services firms",
                "E-commerce and retail brands",
                "Healthcare and clinics",
                "Education and training providers",
            ]
        }
        "software-development" | "custom-software-development" | "crm-software-development" => {
            vec![
                "Professional services firms",
                "Education and training providers",
                "Healthcare and clinics",
                "Retail and distribution businesses",
                "Operationally complex SMEs",
            ]
        }
        "inventory-management-software" => vec![
            "Retail businesses",
            "Wholesale and distribution teams",
            "Pharmacies and healthcare suppliers",
            "Multi-branch stores",
            "Service operations with stocked items",
        ],
        "booking-system-development" | "mobile-app-development" => vec![
            "Healthcare and clinics",
            "Education and training providers",
            "Hospitality and travel services",
            "Professional services firms",
            "Field-service businesses",
        ],
        "hosting-domain-cloud-services" => vec![
            "Corporate websites",
            "Software platforms",
            "Campaign microsites",
            "Growing digital businesses",
            "Multi-environment teams",
        ],
        "ai-automation-solutions" | "it-consultation-digital-transformation" => vec![
            "Operationally growing SMEs",
            "Professional services firms",
            "Education and healthcare organizations",
            "Retail and distribution teams",
            "Businesses modernizing internal workflows",
        ],
        _ => vec![
            "Professional services firms",
            "Retail and distribution businesses",
            "Education and training providers",
            "Healthcare and clinics",
            "Growing digital businesses",
        ],
    }
}

fn technologies_for(slug: &str) -> Vec<&'static str> {
    match slug {
        "web-development" | "website-redesign-services" => vec![
            "Rust and Axum application stacks",
            "Askama templating systems",
            "Responsive front-end engineering",
            "SEO metadata and structured data",
            "Analytics and conversion tracking setup",
        ],
        "website-maintenance-services" | "hosting-domain-cloud-services" => vec![
            "Cloud hosting environments",
            "Deployment and DNS management",
            "Performance monitoring and backups",
            "Security hardening basics",
            "Version-controlled release workflows",
        ],
        "seo-search-growth" | "local-seo-services" => vec![
            "Technical SEO audits",
            "Schema and metadata implementation",
            "Internal linking architecture",
            "Local search signal optimization",
            "Content and page-intent planning",
        ],
        "digital-marketing" | "google-ads-agency" => vec![
            "Google Ads campaign management",
            "Landing page optimization",
            "Conversion tracking setup",
            "Audience and keyword research",
            "Performance reporting workflows",
        ],
        "software-development"
        | "custom-software-development"
        | "crm-software-development"
        | "inventory-management-software"
        | "booking-system-development" => vec![
            "Role-based web application architecture",
            "Admin dashboards and reporting layers",
            "API and system integrations",
            "Workflow and database modeling",
            "Phased product delivery practices",
        ],
        "mobile-app-development" => vec![
            "Android and iOS delivery planning",
            "Cross-platform mobile architecture",
            "API integration for app workflows",
            "User-flow prototyping and QA",
            "Admin and backend coordination",
        ],
        "ai-automation-solutions" => vec![
            "Workflow automation design",
            "Form and CRM integrations",
            "Lead-routing logic",
            "Operational dashboard support",
            "AI-assisted process design",
        ],
        "it-consultation-digital-transformation" => vec![
            "Technology roadmap planning",
            "System architecture reviews",
            "Workflow redesign analysis",
            "Platform selection guidance",
            "Implementation sequencing support",
        ],
        _ => vec![
            "Discovery and roadmap planning",
            "Responsive digital delivery",
            "Workflow-aware implementation",
            "Performance and quality control",
            "Scalable technical foundations",
        ],
    }
}

fn solution_links_for(slug: &str) -> Vec<ServiceLink> {
    match slug {
        "web-development" | "website-redesign-services" => vec![
            service_link(
                "Business Website Package",
                "/packages/business-website-package",
                "A packaged starting point for companies that need a stronger business website with a clear delivery scope.",
            ),
            service_link(
                "E-commerce Development Company",
                "/solutions/ecommerce-development-company",
                "For businesses that need product catalogues, online ordering, and operational e-commerce support.",
            ),
            service_link(
                "Free Website SEO Audit",
                "/free-website-seo-audit",
                "A practical conversion path for teams evaluating why the current website is underperforming.",
            ),
        ],
        "seo-search-growth" | "local-seo-services" | "google-ads-agency" | "digital-marketing" => {
            vec![
                service_link(
                    "Business SEO Offer",
                    "/packages/business-seo-offer",
                    "A structured entry point for businesses that want focused SEO work tied to measurable growth priorities.",
                ),
                service_link(
                    "Free Website SEO Audit",
                    "/free-website-seo-audit",
                    "Useful for identifying the technical and content issues weakening current search visibility.",
                ),
                service_link(
                    "Website Redesign Services",
                    "/services/website-redesign-services",
                    "Useful when weak structure, outdated messaging, or conversion friction are limiting marketing performance.",
                ),
            ]
        }
        "inventory-management-software" => vec![
            service_link(
                "POS System Development",
                "/solutions/pos-system-development",
                "Natural next step when stock control and sales operations need to work from the same source of truth.",
            ),
            service_link(
                "E-commerce Development Company",
                "/solutions/ecommerce-development-company",
                "Relevant when inventory needs to support online ordering and product availability sync.",
            ),
            service_link(
                "Custom Software Development",
                "/services/custom-software-development",
                "Broader solution path for businesses that need inventory as one module inside a larger internal platform.",
            ),
        ],
        "booking-system-development" => vec![
            service_link(
                "Mobile App Development",
                "/services/mobile-app-development",
                "Useful when bookings need a customer-facing or staff-facing mobile layer beyond the web experience.",
            ),
            service_link(
                "CRM Software Development",
                "/services/crm-software-development",
                "Helpful when booking data should feed customer records, follow-up workflows, or service operations.",
            ),
            service_link(
                "Custom Software Development",
                "/services/custom-software-development",
                "Relevant when booking is only one part of a broader operational system.",
            ),
        ],
        "crm-software-development" => vec![
            service_link(
                "Booking System Development",
                "/services/booking-system-development",
                "For businesses that need CRM records and scheduling logic to work together.",
            ),
            service_link(
                "AI & Automation Solutions",
                "/services/ai-automation-solutions",
                "Useful for routing leads, follow-up tasks, and repetitive handling after the CRM core is in place.",
            ),
            service_link(
                "IT Consultation",
                "/services/it-consultation-digital-transformation",
                "Useful when CRM planning is part of a larger digital transformation roadmap.",
            ),
        ],
        _ => vec![
            service_link(
                "Request a Quote",
                "/request-quote",
                "Move from research into a scoped commercial conversation about your exact requirements.",
            ),
            service_link(
                "Case Studies",
                "/case-studies",
                "Review implementation proof across websites, software, SEO, and growth-focused projects.",
            ),
            service_link(
                "Industries",
                "/industries",
                "See how LKProfessionals adapts delivery priorities across sectors and operating models.",
            ),
        ],
    }
}

fn case_study_links_for(slug: &str) -> Vec<ServiceLink> {
    match slug {
        "web-development" | "website-redesign-services" | "website-maintenance-services" => vec![
            service_link(
                "Website and Digital Presence Case Studies",
                "/case-studies?category=websites",
                "Relevant proof for businesses comparing site quality, trust, and conversion improvements.",
            ),
            service_link(
                "SEO and Search Visibility Projects",
                "/case-studies?category=seo",
                "Examples of how stronger structure and optimization work together in live projects.",
            ),
        ],
        "software-development"
        | "custom-software-development"
        | "crm-software-development"
        | "inventory-management-software"
        | "booking-system-development"
        | "mobile-app-development" => vec![
            service_link(
                "Software and Platform Case Studies",
                "/case-studies?category=software",
                "Implementation proof for internal systems, platforms, dashboards, and business workflow products.",
            ),
            service_link(
                "Automation and Operations Projects",
                "/case-studies?category=automation",
                "Useful when the buyer cares about process improvement, operational control, or integration logic.",
            ),
        ],
        _ => vec![
            service_link(
                "Full Case Study Library",
                "/case-studies",
                "Browse portfolio proof across service lines, sectors, and commercial use cases.",
            ),
            service_link(
                "Industry Delivery Examples",
                "/industries",
                "See how project direction changes based on the operational realities of each sector.",
            ),
        ],
    }
}

fn insight_links_for(slug: &str) -> Vec<ServiceLink> {
    match slug {
        "seo-search-growth" => vec![
            service_link(
                "On-page SEO for service businesses",
                "/insights/on-page-seo-for-service-businesses-how-to-write-pages-that-rank-and-convert",
                "A practical look at how service pages should be structured for both rankings and conversion.",
            ),
            service_link(
                "How FAQs help service pages rank and convert",
                "/insights/how-do-faqs-help-service-pages-rank-and-convert",
                "Explains why commercially useful FAQs strengthen both SEO coverage and buyer confidence.",
            ),
            service_link(
                "How internal links help AI and search engines understand your website",
                "/insights/how-do-internal-links-help-ai-and-search-engines-understand-your-website",
                "Relevant for building stronger topical authority and clearer page relationships.",
            ),
        ],
        "local-seo-services" => vec![
            service_link(
                "Why service-area pages matter for local SEO",
                "/insights/why-do-service-area-pages-matter-for-local-seo",
                "Useful context for how local relevance should be built without producing weak duplicate pages.",
            ),
            service_link(
                "How to write location pages without thin content",
                "/insights/how-do-you-write-location-pages-without-thin-content",
                "Explains how to scale local SEO pages while preserving quality and search intent fit.",
            ),
            service_link(
                "What is the difference between local SEO and national SEO",
                "/insights/what-is-the-difference-between-local-seo-and-national-seo",
                "Clarifies when local search strategy should take priority over broader organic targeting.",
            ),
        ],
        "website-redesign-services" | "web-development" => vec![
            service_link(
                "How to plan a website redesign without losing rankings",
                "/insights/how-do-you-plan-a-website-redesign-without-losing-rankings",
                "A relevant guide for businesses balancing modernization with SEO continuity.",
            ),
            service_link(
                "What should be on a homepage above the fold for a B2B IT brand",
                "/insights/what-should-be-on-a-homepage-above-the-fold-for-a-b2b-it-brand",
                "Useful for understanding how trust and conversion begin before the scroll.",
            ),
            service_link(
                "What makes a software company website feel trustworthy",
                "/insights/what-makes-a-software-company-website-feel-trustworthy",
                "Explores the credibility signals buyers expect from a serious digital services firm.",
            ),
        ],
        "website-maintenance-services" | "hosting-domain-cloud-services" => vec![
            service_link(
                "Website maintenance in Sri Lanka",
                "/insights/website-maintenance-in-sri-lanka-why-launch-is-only-the-beginning",
                "A useful explainer on why websites need support after launch to preserve trust and performance.",
            ),
            service_link(
                "How fast hosting supports better SEO and conversions",
                "/insights/how-fast-hosting-supports-better-seo-and-better-conversions",
                "Connects infrastructure decisions directly to search visibility and enquiry quality.",
            ),
            service_link(
                "Security basics for business websites",
                "/insights/security-basics-for-business-websites-what-hosting-alone-will-not-solve",
                "Clarifies what businesses should expect beyond a basic hosting package.",
            ),
        ],
        "crm-software-development" => vec![
            service_link(
                "CRM development vs CRM setup",
                "/insights/crm-development-vs-crm-setup-which-path-is-right-for-your-team",
                "Useful for buyers deciding whether they need custom development or a lighter implementation path.",
            ),
            service_link(
                "What questions buyers ask before hiring a software company",
                "/insights/what-questions-do-buyers-ask-before-hiring-a-software-company",
                "Helpful for teams evaluating vendor fit and software delivery discipline.",
            ),
            service_link(
                "Content strategy for software companies",
                "/insights/content-strategy-for-software-companies-how-insights-support-commercial-rankings",
                "Shows how authority-building content supports harder commercial software keywords.",
            ),
        ],
        "inventory-management-software" => vec![
            service_link(
                "Inventory management software in Sri Lanka",
                "/insights/inventory-management-software-in-sri-lanka-when-spreadsheets-stop-being-enough",
                "Explains the operational tipping point where manual stock tracking starts holding the business back.",
            ),
            service_link(
                "Barcode inventory and reporting",
                "/insights/barcode-inventory-and-reporting-the-pos-features-that-actually-save-time",
                "Relevant when the buyer is evaluating stock handling and reporting workflows together.",
            ),
            service_link(
                "E-commerce and POS integration",
                "/insights/e-commerce-and-pos-integration-why-retail-businesses-need-one-source-of-truth",
                "Useful context when inventory, sales, and online channels must stay aligned.",
            ),
        ],
        "booking-system-development" => vec![
            service_link(
                "Booking system development in Sri Lanka",
                "/insights/booking-system-development-in-sri-lanka-what-hospitality-and-service-brands-need",
                "Useful for buyers thinking through schedule complexity, customer expectations, and operational control.",
            ),
            service_link(
                "Booking apps for service companies",
                "/insights/booking-apps-for-service-companies-what-needs-to-be-simple-from-day-one",
                "Explains the customer experience details that make booking software easier to adopt.",
            ),
            service_link(
                "Customer portal vs mobile app",
                "/insights/customer-portal-vs-mobile-app-the-better-first-step-for-some-businesses",
                "Helpful when deciding whether booking should live on the web, in an app, or across both.",
            ),
        ],
        _ => vec![
            service_link(
                "How many service pages should a growing IT company publish",
                "/insights/how-many-service-pages-should-a-growing-it-company-publish",
                "Explains the role of focused commercial pages in building long-term topical authority.",
            ),
            service_link(
                "How do you turn search traffic into qualified leads",
                "/insights/how-do-you-turn-search-traffic-into-qualified-leads",
                "Useful for understanding how page quality and conversion logic must work together.",
            ),
            service_link(
                "What should be included on a high-converting request-a-quote form",
                "/insights/what-should-be-included-on-a-high-converting-request-a-quote-form",
                "Relevant for businesses trying to improve lead quality after attracting more traffic.",
            ),
        ],
    }
}
