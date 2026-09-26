#[derive(Debug, Clone)]
pub struct PackageFaq {
    pub question: &'static str,
    pub answer: &'static str,
}

#[derive(Debug, Clone)]
pub struct RelatedPackage {
    pub label: &'static str,
    pub name: &'static str,
    pub href: &'static str,
    pub note: &'static str,
}

#[derive(Debug, Clone)]
pub struct PackageLink {
    pub title: &'static str,
    pub href: &'static str,
    pub note: &'static str,
}

#[derive(Debug, Clone)]
pub struct PackageStep {
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Debug, Clone)]
pub struct WebPackage {
    pub family_name: &'static str,
    pub family_url: &'static str,
    pub family_label: &'static str,
    pub slug: &'static str,
    pub name: &'static str,
    pub short_name: &'static str,
    pub catalog_name: &'static str,
    pub item_code: &'static str,
    pub category: &'static str,
    pub price: &'static str,
    pub price_number: &'static str,
    pub positioning: &'static str,
    pub audience: Vec<&'static str>,
    pub core_message: &'static str,
    pub description: &'static str,
    pub hero_suffix: &'static str,
    pub delivery_label: &'static str,
    pub inclusions: Vec<&'static str>,
    pub differentiator: &'static str,
    pub business_problem: &'static str,
    pub outcomes: Vec<&'static str>,
    pub example_structure: Vec<&'static str>,
    pub structure_heading: &'static str,
    pub process: Vec<PackageStep>,
    pub process_heading: &'static str,
    pub addons: Vec<&'static str>,
    pub pricing_note: &'static str,
    pub faqs: Vec<PackageFaq>,
    pub related: Vec<RelatedPackage>,
    pub meta_title: &'static str,
    pub meta_description: &'static str,
    pub icon: &'static str,
}

#[derive(Debug, Clone)]
pub struct WebPackageFamily {
    pub name: &'static str,
    pub short_name: &'static str,
    pub url: &'static str,
    pub meta_title: &'static str,
    pub meta_description: &'static str,
    pub schema_description: &'static str,
    pub hero_title: &'static str,
    pub hero_description: &'static str,
    pub whatsapp_text: &'static str,
    pub how_it_works: Vec<&'static str>,
    pub range_heading: &'static str,
    pub guidance_title: &'static str,
    pub guidance_items: Vec<&'static str>,
    pub useful_links: Vec<PackageLink>,
    pub faqs: Vec<PackageFaq>,
    pub final_cta_heading: &'static str,
    pub final_cta_description: &'static str,
    pub lifecycle_steps: Vec<PackageStep>,
    pub packages: Vec<WebPackage>,
}

impl WebPackage {
    pub fn url(&self) -> String {
        format!("{}/{}", self.family_url, self.slug)
    }

    pub fn canonical_url(&self) -> String {
        format!("https://lkprofessionals.com{}", self.url())
    }

    pub fn quote_url(&self) -> String {
        format!("/request-quote?package={}&source={}", self.slug, self.url())
    }

    pub fn whatsapp_url(&self) -> String {
        let message = format!(
            "Hello LKProfessionals, I want to discuss the {} starting from {}. Source: {}",
            self.name,
            self.price,
            self.url()
        )
        .replace(' ', "%20")
        .replace(':', "%3A")
        .replace('/', "%2F")
        .replace(',', "%2C")
        .replace('+', "%2B");

        format!("https://wa.me/94761234321?text={message}")
    }
}

pub fn web_design_family() -> WebPackageFamily {
    WebPackageFamily {
        name: "Web Design & Development Packages",
        short_name: "Web Design & Development",
        url: "/packages/web-design-development",
        meta_title: "Web Design & Development Packages Sri Lanka | LKProfessionals",
        meta_description: "Compare LKProfessionals web design and development packages from LKR 10,000+, from single-page websites to corporate, e-commerce, custom, and multilingual platforms.",
        schema_description: "LKProfessionals web design and development packages for businesses that need websites, e-commerce, corporate platforms, custom development, or multilingual web architecture.",
        hero_title: "Choose a website package that fits where your business is going.",
        hero_description: "LKProfessionals packages progress from affordable online presence builds to growth websites, e-commerce, corporate platforms, bespoke web engineering, and international multilingual websites.",
        whatsapp_text: "Hello%20LKProfessionals%2C%20I%20want%20help%20choosing%20a%20web%20design%20and%20development%20package.",
        how_it_works: vec![
            "Each price is a starting point, not a promise that every possible feature fits inside one fixed scope.",
            "Choose by business stage, website purpose, page depth, integrations, and future growth needs.",
            "If the requirements do not fit neatly, LKProfessionals can prepare a custom quotation.",
        ],
        range_heading: "From first presence to custom web platform.",
        guidance_title: "Start with the business goal.",
        guidance_items: vec![
            "Need credibility fast? Start with Digital Footprint or LaunchPad.",
            "Need nearby customers? Review Local Spark.",
            "Need leads, corporate authority, commerce, or international structure? Compare Growth Engine, Flagship, Omnichannel Commerce, and Global Reach.",
            "Need users, workflows, dashboards, databases, or integrations? Apex Custom is the right conversation.",
        ],
        useful_links: vec![
            package_link(
                "Web Development",
                "/services/web-development",
                "See the broader service capability behind these packages.",
            ),
            package_link(
                "Case Studies",
                "/case-studies",
                "Review selected work and implementation context.",
            ),
            package_link(
                "Custom Software",
                "/services/custom-software-development",
                "Useful for Apex Custom and platform-style requirements.",
            ),
            package_link(
                "AI Search Growth",
                "/packages/business-seo-offer",
                "Consider after launch when visibility becomes the priority.",
            ),
        ],
        faqs: vec![
            faq(
                "Which web design package should I choose?",
                "Choose by business stage and complexity. Entry-level presence, first business website, local discovery, growth, redesign, lead generation, e-commerce, corporate authority, custom platform needs, and international expansion each point to a different package.",
            ),
            faq(
                "Are prices fixed?",
                "All listed prices are starting prices. Final pricing depends on pages, content, integrations, languages, migration needs, and custom functionality.",
            ),
            faq(
                "Can LKProfessionals recommend a package?",
                "Yes. Share your business type, current website status, must-have pages, target launch date, and any integrations you need. The team can recommend a package or scope a custom quotation.",
            ),
        ],
        final_cta_heading: "Need a recommendation before choosing?",
        final_cta_description: "Send your business type, existing website link if any, target pages, features, and budget direction. LKProfessionals will help map the requirement to the closest package or prepare a custom quote.",
        lifecycle_steps: website_process(),
        packages: vec![
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "digital-footprint",
                name: "Digital Footprint",
                short_name: "Digital Footprint",
                catalog_name: "Digital Footprint - Web Design Package",
                item_code: "LKP-WEB-DF-001",
                category: "Entry-level professional website",
                price: "LKR 10,000+",
                price_number: "10000",
                positioning: "Entry-level professional online presence.",
                audience: vec![
                    "Freelancers",
                    "Solo entrepreneurs",
                    "Consultants",
                    "Professionals",
                    "Personal portfolios",
                    "Small service providers",
                ],
                core_message: "Build your professional presence online.",
                description: "Digital Footprint is an affordable professional website package for freelancers, solo entrepreneurs, professionals, and small businesses establishing their first online presence.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Professionally designed single-page website",
                    "Responsive desktop, tablet, and mobile design",
                    "About or introduction section",
                    "Services or expertise section",
                    "Contact section",
                    "WhatsApp integration",
                    "Social media links",
                    "Basic on-page SEO",
                    "Performance-focused development",
                    "SSL/HTTPS configuration",
                ],
                differentiator: "A focused single-page website that gives a professional, searchable destination without a larger initial build.",
                business_problem: "You need a credible online presence customers can visit, but a multi-page website would be more than the current requirement.",
                outcomes: vec![
                    "A professional page you can share from profiles, proposals, and social channels",
                    "Clear contact and WhatsApp paths for interested visitors",
                    "A practical foundation that can expand into a larger website later",
                ],
                example_structure: vec![
                    "Hero with your offer and primary contact action",
                    "About or professional introduction",
                    "Services, expertise, or portfolio summary",
                    "Contact, WhatsApp, and social links",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Domain and hosting",
                    "Additional pages",
                    "Advanced forms",
                    "Booking, payment, or custom functionality",
                ],
                pricing_note: "Starting from LKR 10,000. Domain, hosting, additional pages, and advanced functionality can be added separately after scope review.",
                faqs: vec![
                    faq(
                        "Is this only one page?",
                        "Yes. Digital Footprint is designed as a professional single-page website. Additional pages can be quoted separately.",
                    ),
                    faq(
                        "Is domain and hosting included?",
                        "Domain and hosting can be added separately for this entry-level package.",
                    ),
                    faq(
                        "Can I upgrade later?",
                        "Yes. This package can grow into LaunchPad, Momentum, or a custom website when your requirements expand.",
                    ),
                ],
                related: vec![related(
                    "Upgrade",
                    "LaunchPad",
                    "/packages/web-design-development/launchpad",
                    "Move to a complete first business website with multiple pages and hosting included.",
                )],
                meta_title: "Digital Footprint Web Design Package Sri Lanka | LKProfessionals",
                meta_description: "Digital Footprint starts from LKR 10,000 for freelancers, consultants, portfolios, and small service providers that need a professional single-page website.",
                icon: "fa-id-badge",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "launchpad",
                name: "LaunchPad",
                short_name: "LaunchPad",
                catalog_name: "LaunchPad - Business Website Package",
                item_code: "LKP-WEB-LP-002",
                category: "Complete first business website",
                price: "LKR 30,000+",
                price_number: "30000",
                positioning: "A complete first business website for startups and small businesses.",
                audience: vec![
                    "Startups",
                    "Small businesses",
                    "Professional service providers",
                    "Growing companies",
                ],
                core_message: "Launch your business with a professional website built for growth.",
                description: "LaunchPad gives startups and small businesses a structured, responsive business website with the essentials needed for a credible launch.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Up to 6 pages",
                    "Modern custom responsive design",
                    "Mobile, tablet, and desktop optimisation",
                    "Home, About, Services, and Contact pages",
                    "Contact and enquiry functionality",
                    "WhatsApp integration",
                    "Social media integration",
                    "Basic on-page SEO",
                    "Blog functionality",
                    "Administration dashboard",
                    "SSL/HTTPS",
                    "Performance optimisation",
                    "Domain and hosting for first year",
                ],
                differentiator: "A complete first business website with domain, hosting, blog, dashboard, and core enquiry flows included.",
                business_problem: "Your business needs more than a profile page: customers need to understand your company, services, and how to enquire.",
                outcomes: vec![
                    "A launch-ready website with clear service and contact pages",
                    "A content foundation that can support future SEO and blog updates",
                    "A professional platform that can expand as the company grows",
                ],
                example_structure: vec![
                    "Home",
                    "About",
                    "Services",
                    "Service detail or business-specific page",
                    "Blog",
                    "Contact",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Extra pages",
                    "Copywriting",
                    "Advanced SEO",
                    "Booking or payment features",
                    "Ongoing maintenance",
                ],
                pricing_note: "Starting from LKR 30,000 with domain and hosting for the first year included within the defined package scope.",
                faqs: vec![
                    faq(
                        "How many pages are included?",
                        "LaunchPad includes up to 6 pages.",
                    ),
                    faq(
                        "Is a blog included?",
                        "Yes. Blog functionality and administration dashboard access are included.",
                    ),
                    faq(
                        "Who is LaunchPad best for?",
                        "It is best for startups, small businesses, professional services, and growing companies launching a complete first website.",
                    ),
                ],
                related: vec![
                    related(
                        "Simpler",
                        "Digital Footprint",
                        "/packages/web-design-development/digital-footprint",
                        "Use this if a single-page online presence is enough.",
                    ),
                    related(
                        "Local alternative",
                        "Local Spark",
                        "/packages/web-design-development/local-spark",
                        "Use this if nearby customer discovery is the main priority.",
                    ),
                    related(
                        "Upgrade",
                        "Momentum",
                        "/packages/web-design-development/momentum",
                        "Use this when you need a stronger platform with more pages and growth structure.",
                    ),
                ],
                meta_title: "LaunchPad Business Website Package Sri Lanka | LKProfessionals",
                meta_description: "LaunchPad starts from LKR 30,000 for startups and small businesses needing a complete responsive website with hosting, blog, SEO basics, and dashboard.",
                icon: "fa-rocket",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "local-spark",
                name: "Local Spark",
                short_name: "Local Spark",
                catalog_name: "Local Spark - Local Business Website Package",
                item_code: "LKP-WEB-LS-003",
                category: "Local business website",
                price: "LKR 40,000+",
                price_number: "40000",
                positioning: "A website designed to help location-based businesses attract nearby customers.",
                audience: vec![
                    "Restaurants",
                    "Cafes",
                    "Salons",
                    "Local retailers",
                    "Repair services",
                    "Construction businesses",
                    "Tuition centres",
                    "Professional services",
                    "Location-based businesses",
                ],
                core_message: "Get found locally. Turn searches into real customers.",
                description: "Local Spark is built for location-based businesses that need service-area content, map visibility, click-to-call paths, WhatsApp, and local SEO foundations.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Up to 8 pages",
                    "Custom responsive design",
                    "Mobile, tablet, and desktop optimisation",
                    "Service-focused pages",
                    "Location and service-area content",
                    "Google Maps integration",
                    "WhatsApp integration",
                    "Click-to-call functionality",
                    "Contact and enquiry forms",
                    "Social media integration",
                    "Local SEO foundation",
                    "Basic on-page SEO",
                    "Blog functionality",
                    "Administration dashboard",
                    "SSL/HTTPS",
                    "Performance optimisation",
                    "Domain and hosting for first year",
                ],
                differentiator: "Local SEO, service-area content, map integration, and mobile contact actions are treated as core requirements.",
                business_problem: "Nearby customers search before they call or visit, and your website needs to make location, services, and contact actions obvious.",
                outcomes: vec![
                    "Clear local service positioning",
                    "Easier phone, WhatsApp, map, and enquiry actions",
                    "A stronger base for local search visibility without ranking guarantees",
                ],
                example_structure: vec![
                    "Home with local value proposition",
                    "About",
                    "Services",
                    "Service areas or location page",
                    "Gallery or menu",
                    "Blog",
                    "Contact with map",
                    "Business-specific page",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Google Business Profile support",
                    "Menu/catalog sections",
                    "Additional service-area pages",
                    "Review collection workflows",
                    "Local SEO campaign",
                ],
                pricing_note: "Starting from LKR 40,000. Final scope depends on number of locations, service areas, content needs, and integrations.",
                faqs: vec![
                    faq(
                        "Does Local Spark guarantee local rankings?",
                        "No. It creates a local SEO foundation, but search rankings depend on many factors outside a website build alone.",
                    ),
                    faq(
                        "Is Google Maps included?",
                        "Yes. Google Maps integration is included where a business location or service area is relevant.",
                    ),
                    faq(
                        "Can this work for service-area businesses?",
                        "Yes. It can support location and service-area content for businesses that visit customers instead of operating only from one shop.",
                    ),
                ],
                related: vec![
                    related(
                        "General alternative",
                        "LaunchPad",
                        "/packages/web-design-development/launchpad",
                        "Use this if local search is not the main requirement.",
                    ),
                    related(
                        "Growth alternative",
                        "Momentum",
                        "/packages/web-design-development/momentum",
                        "Use this when the business needs a broader growth website.",
                    ),
                ],
                meta_title: "Local Spark Local Business Website Package | LKProfessionals",
                meta_description: "Local Spark starts from LKR 40,000 for restaurants, salons, retailers, repair services, tuition centres, and other location-based businesses.",
                icon: "fa-location-dot",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "momentum",
                name: "Momentum",
                short_name: "Momentum",
                catalog_name: "Momentum - Growth Website Package",
                item_code: "LKP-WEB-MO-004",
                category: "Growth website",
                price: "LKR 60,000+",
                price_number: "60000",
                positioning: "A stronger digital platform for businesses that have moved beyond a basic website.",
                audience: vec![
                    "Growing businesses",
                    "SMEs",
                    "Professional service companies",
                    "Established companies",
                    "Businesses expanding operations",
                ],
                core_message: "Accelerate your business growth online.",
                description: "Momentum supports businesses that need a more substantial website with richer service presentation, conversion-focused structure, analytics, and a scalable content base.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Up to 12 pages",
                    "Bespoke brand-focused design",
                    "Responsive optimisation",
                    "Advanced service or product presentation",
                    "Conversion-focused structure",
                    "Custom contact and enquiry forms",
                    "WhatsApp integration",
                    "Social media integration",
                    "Blog and content management",
                    "Administration dashboard",
                    "On-page SEO foundation",
                    "Google Analytics integration",
                    "Google Search Console setup",
                    "Performance optimisation",
                    "SSL/HTTPS",
                    "Domain and hosting for first year",
                ],
                differentiator: "A stronger business website shaped around growth, analytics, and richer service presentation.",
                business_problem: "A simple website is no longer enough because your services, buyer journey, and content needs have become more complex.",
                outcomes: vec![
                    "Clearer service architecture for buyers",
                    "Better enquiry paths across the site",
                    "Analytics and Search Console foundations for future improvement",
                ],
                example_structure: vec![
                    "Home",
                    "About",
                    "Core services",
                    "Individual service pages",
                    "Industries or solutions",
                    "Blog",
                    "Contact",
                    "Conversion landing page",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "SEO content program",
                    "CRM integration",
                    "Advanced conversion tracking",
                    "Case study pages",
                    "Extra service landing pages",
                ],
                pricing_note: "Starting from LKR 60,000. Final pricing depends on page count, content complexity, forms, and tracking needs.",
                faqs: vec![
                    faq(
                        "How is Momentum different from LaunchPad?",
                        "Momentum includes more page capacity, richer service presentation, analytics setup, and a stronger growth-oriented structure.",
                    ),
                    faq(
                        "Is Google Analytics included?",
                        "Yes. Google Analytics and Google Search Console setup are included.",
                    ),
                    faq(
                        "Can Momentum support future SEO work?",
                        "Yes. It creates a practical foundation for service pages, blog content, technical SEO, and future campaigns.",
                    ),
                ],
                related: vec![
                    related(
                        "Redesign alternative",
                        "Vanguard",
                        "/packages/web-design-development/vanguard",
                        "Use this if you already have an outdated website that needs modernisation.",
                    ),
                    related(
                        "Conversion upgrade",
                        "The Growth Engine",
                        "/packages/web-design-development/growth-engine",
                        "Use this if lead generation is the main priority.",
                    ),
                ],
                meta_title: "Momentum Growth Website Package Sri Lanka | LKProfessionals",
                meta_description: "Momentum starts from LKR 60,000 for SMEs and growing businesses that need a stronger website with analytics, SEO foundations, and conversion-focused structure.",
                icon: "fa-chart-line",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "vanguard",
                name: "Vanguard",
                short_name: "Vanguard",
                catalog_name: "Vanguard - Premium Website Redesign Package",
                item_code: "LKP-WEB-VG-005",
                category: "Premium website redesign",
                price: "LKR 80,000+",
                price_number: "80000",
                positioning: "Premium redesign and modernisation for businesses that already have a website.",
                audience: vec![
                    "Established businesses",
                    "Professional firms",
                    "Organisations",
                    "Established brands",
                    "Companies with outdated websites",
                ],
                core_message: "You already have a website. Now build one worthy of the business you have become.",
                description: "Vanguard modernises outdated websites with premium UI/UX, improved navigation, migration planning, SEO foundations, and redirect planning for important existing URLs.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Complete website redesign",
                    "Up to 15 pages",
                    "Premium custom UI/UX",
                    "Brand-focused visual direction",
                    "Responsive optimisation",
                    "Existing content migration",
                    "Improved information architecture and navigation",
                    "Conversion-focused layouts",
                    "Advanced enquiry forms",
                    "WhatsApp and social integration",
                    "Blog and content management",
                    "Administration dashboard",
                    "Technical SEO foundation",
                    "On-page SEO foundation",
                    "Google Analytics",
                    "Google Search Console",
                    "Core Web Vitals and performance optimisation",
                    "SSL/HTTPS",
                    "Domain and hosting for first year",
                    "Redirect planning for important existing URLs",
                ],
                differentiator: "Redesign strategy, content migration, URL care, and modern UI/UX are handled together.",
                business_problem: "Your current website no longer reflects your business quality, and replacing it needs planning so important content and URLs are not ignored.",
                outcomes: vec![
                    "A more current and credible digital presence",
                    "Improved navigation and conversion paths",
                    "A planned transition from old pages to new ones",
                ],
                example_structure: vec![
                    "Redesigned home",
                    "About and team",
                    "Services and service detail pages",
                    "Case studies or proof pages",
                    "Insights or blog",
                    "Contact and enquiry workflows",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Full content rewrite",
                    "Brand identity refresh",
                    "Advanced migration audit",
                    "CRM integration",
                    "Ongoing SEO retainer",
                ],
                pricing_note: "Starting from LKR 80,000. Final pricing depends on existing website size, migration requirements, redesign depth, and integrations.",
                faqs: vec![
                    faq(
                        "Will old URLs be considered?",
                        "Yes. Redirect planning for important existing URLs is included so the redesign has a cleaner transition path.",
                    ),
                    faq(
                        "Is this for new websites?",
                        "Vanguard is mainly for businesses that already have a website and need a premium redesign or modernisation.",
                    ),
                    faq(
                        "Can you migrate existing content?",
                        "Yes. Existing content migration is included within the agreed page and content scope.",
                    ),
                ],
                related: vec![
                    related(
                        "Lead generation alternative",
                        "The Growth Engine",
                        "/packages/web-design-development/growth-engine",
                        "Use this if the redesign is mainly about enquiries and conversion.",
                    ),
                    related(
                        "Corporate upgrade",
                        "The Flagship",
                        "/packages/web-design-development/flagship",
                        "Use this if the organisation needs a deeper corporate website.",
                    ),
                ],
                meta_title: "Vanguard Website Redesign Package Sri Lanka | LKProfessionals",
                meta_description: "Vanguard starts from LKR 80,000 for established businesses that need premium website redesign, migration planning, SEO foundations, and modern UI/UX.",
                icon: "fa-compass-drafting",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "growth-engine",
                name: "The Growth Engine",
                short_name: "Growth Engine",
                catalog_name: "The Growth Engine - Lead Generation Website Package",
                item_code: "LKP-WEB-GE-006",
                category: "Lead generation website",
                price: "LKR 120,000+",
                price_number: "120000",
                positioning: "A website engineered primarily around lead generation and conversion.",
                audience: vec![
                    "Service businesses",
                    "B2B companies",
                    "Agencies",
                    "Professional firms",
                    "Established businesses dependent on online enquiries",
                ],
                core_message: "Your website should not just get visitors. It should generate business.",
                description: "The Growth Engine is for service-led businesses that need dedicated landing pages, strong CTAs, lead capture flows, CRM-ready structure, tracking, and SEO foundations.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Up to 20 pages",
                    "Conversion-focused custom website",
                    "Premium UI/UX",
                    "Responsive optimisation",
                    "Dedicated service landing pages",
                    "Strategic CTAs",
                    "Lead capture forms",
                    "Enquiry forms",
                    "WhatsApp integration",
                    "Click-to-call",
                    "CRM-ready lead capture structure",
                    "Blog and content management",
                    "Administration dashboard",
                    "Advanced on-page SEO foundation",
                    "Technical SEO",
                    "Conversion tracking",
                    "Google Analytics",
                    "Google Search Console",
                    "Core Web Vitals and performance optimisation",
                    "SSL/HTTPS",
                    "Domain and hosting for first year",
                ],
                differentiator: "Lead capture, landing pages, tracking, and conversion paths are built into the website architecture.",
                business_problem: "The business depends on enquiries, but the website needs clearer landing pages, CTAs, forms, and tracking before serious optimisation can happen.",
                outcomes: vec![
                    "More structured enquiry journeys",
                    "Tracking foundations for evaluating campaigns and content",
                    "Service pages designed around buyer intent without guaranteeing lead volume",
                ],
                example_structure: vec![
                    "Home",
                    "Core conversion pages",
                    "Dedicated service landing pages",
                    "Industries or use cases",
                    "Resource content",
                    "Lead capture forms",
                    "Contact and WhatsApp paths",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "CRM implementation",
                    "Paid campaign landing pages",
                    "Marketing automation",
                    "Advanced analytics dashboards",
                    "Ongoing SEO or content program",
                ],
                pricing_note: "Starting from LKR 120,000. Scope is shaped around services, lead flows, tracking requirements, and integration depth.",
                faqs: vec![
                    faq(
                        "Does this guarantee leads?",
                        "No. It creates a conversion-focused website foundation, but lead volume depends on traffic, offer, market demand, campaigns, and follow-up.",
                    ),
                    faq(
                        "Can forms connect to a CRM?",
                        "The package includes CRM-ready lead capture structure. Specific CRM integrations are scoped based on the platform and workflow.",
                    ),
                    faq(
                        "Is this suitable for B2B services?",
                        "Yes. It is a strong fit for B2B, agencies, professional firms, and service businesses dependent on online enquiries.",
                    ),
                ],
                related: vec![
                    related(
                        "Corporate alternative",
                        "The Flagship",
                        "/packages/web-design-development/flagship",
                        "Use this if authority and organisational depth matter as much as lead capture.",
                    ),
                    related(
                        "Commerce alternative",
                        "Omnichannel Commerce",
                        "/packages/web-design-development/omnichannel-commerce",
                        "Use this if online selling is the core requirement.",
                    ),
                ],
                meta_title: "Growth Engine Lead Generation Website Package | LKProfessionals",
                meta_description: "The Growth Engine starts from LKR 120,000 for service and B2B businesses needing lead generation website structure, landing pages, tracking, and SEO foundations.",
                icon: "fa-filter-circle-dollar",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "omnichannel-commerce",
                name: "Omnichannel Commerce",
                short_name: "Omnichannel Commerce",
                catalog_name: "Omnichannel Commerce - Advanced E-Commerce Website Package",
                item_code: "LKP-WEB-OC-007",
                category: "Advanced e-commerce website",
                price: "LKR 180,000+",
                price_number: "180000",
                positioning: "Advanced e-commerce infrastructure for businesses serious about selling online.",
                audience: vec![
                    "Retailers",
                    "Product brands",
                    "Wholesalers",
                    "Established stores",
                    "Growing online businesses",
                    "Multi-channel sellers",
                ],
                core_message: "One business. Every channel. Built to sell.",
                description: "Omnichannel Commerce provides custom e-commerce design and development with catalogue, checkout, payments, order management, product SEO foundations, and commerce administration.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Custom e-commerce design and development",
                    "Responsive commerce experience",
                    "Product catalogue",
                    "Category management",
                    "Product variations and attributes",
                    "Shopping cart",
                    "Checkout",
                    "Customer accounts",
                    "Order management",
                    "Inventory management foundation",
                    "Payment gateway integration",
                    "Delivery and shipping configuration",
                    "Promotional codes and discounts",
                    "WhatsApp integration",
                    "Social integration",
                    "Product search and filtering",
                    "E-commerce administration dashboard",
                    "SEO-friendly product and category architecture",
                    "Technical SEO foundation",
                    "On-page SEO foundation",
                    "Google Analytics",
                    "Conversion tracking",
                    "Google Search Console",
                    "SSL/security",
                    "Performance optimisation",
                    "Domain and hosting for first year",
                ],
                differentiator: "Commerce architecture, product management, checkout, payments, shipping, and tracking are handled as one selling platform.",
                business_problem: "Selling online needs more than a catalogue page: customers need product discovery, checkout, payments, delivery information, and order management.",
                outcomes: vec![
                    "A structured online store foundation",
                    "Product and category pages built with SEO-friendly architecture",
                    "Admin tools for products, orders, and commerce operations",
                ],
                example_structure: vec![
                    "Storefront home",
                    "Product categories",
                    "Product detail pages",
                    "Cart and checkout",
                    "Customer account area",
                    "Order administration",
                    "Promotions and delivery information",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Multi-vendor functionality",
                    "POS integration",
                    "ERP integration",
                    "Advanced inventory",
                    "Mobile applications",
                    "Marketplace integrations",
                ],
                pricing_note: "Starting from LKR 180,000. Additions such as POS, ERP, marketplace, multi-vendor, advanced inventory, and mobile apps are quoted separately.",
                faqs: vec![
                    faq(
                        "Are payment gateways included?",
                        "Payment gateway integration is included within the agreed payment provider and technical scope.",
                    ),
                    faq(
                        "Can this support product variations?",
                        "Yes. Product variations and attributes are part of the package foundation.",
                    ),
                    faq(
                        "Are ERP or POS integrations included?",
                        "ERP, POS, marketplace, and advanced inventory integrations are separately quoted because requirements vary widely.",
                    ),
                ],
                related: vec![
                    related(
                        "Custom commerce requirements",
                        "Apex Custom",
                        "/packages/web-design-development/apex-custom",
                        "Use this if commerce needs move beyond a standard online store.",
                    ),
                    related(
                        "Connected commerce platform",
                        "The Ecosystem",
                        "/packages/app-software-development/the-ecosystem",
                        "Use this when commerce needs mobile apps, portals, dashboards, and backend operations working together.",
                    ),
                    related(
                        "Commerce platform business",
                        "Platform Forge",
                        "/packages/app-software-development/platform-forge",
                        "Use this when commerce requirements become a multi-user SaaS, marketplace, or subscription platform.",
                    ),
                ],
                meta_title: "Omnichannel Commerce E-Commerce Website Package | LKProfessionals",
                meta_description: "Omnichannel Commerce starts from LKR 180,000 for retailers, wholesalers, and product brands needing advanced e-commerce design and development.",
                icon: "fa-cart-shopping",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "flagship",
                name: "The Flagship",
                short_name: "Flagship",
                catalog_name: "The Flagship - Premium Corporate Website Package",
                item_code: "LKP-WEB-FS-008",
                category: "Premium corporate website",
                price: "LKR 250,000+",
                price_number: "250000",
                positioning: "A substantial, authoritative corporate digital presence.",
                audience: vec![
                    "Corporations",
                    "Large businesses",
                    "Institutions",
                    "NGOs",
                    "Engineering and construction companies",
                    "Professional firms",
                    "Established brands",
                ],
                core_message: "Your organisation has a reputation. Your website should carry it.",
                description: "The Flagship is a premium corporate website package for organisations that need deep information architecture, advanced CMS, structured data, resource centres, careers, projects, and stakeholder-friendly enquiry flows.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Up to 30 pages",
                    "Bespoke corporate website",
                    "Premium custom UI/UX",
                    "Corporate brand visual system",
                    "Advanced responsive design",
                    "Complex information architecture",
                    "Services, industries, and solutions sections",
                    "Projects, portfolio, or case studies",
                    "Leadership and team profiles",
                    "Careers and recruitment",
                    "News, insights, or resource centre",
                    "Advanced enquiry workflows",
                    "WhatsApp and social integration",
                    "Advanced CMS",
                    "Administration dashboard",
                    "Technical SEO",
                    "On-page SEO",
                    "Structured data",
                    "Google Analytics",
                    "Google Search Console",
                    "Conversion and event tracking",
                    "Core Web Vitals and performance optimisation",
                    "SSL/security",
                    "Domain and hosting for first year",
                ],
                differentiator: "Corporate-grade structure for authority, recruitment, projects, resources, and multiple stakeholder journeys.",
                business_problem: "The organisation has complex services, proof, teams, resources, and audiences that need a substantial website rather than a simple brochure site.",
                outcomes: vec![
                    "A stronger corporate presence for buyers, partners, recruits, and stakeholders",
                    "Scalable content areas for projects, insights, and careers",
                    "Structured data and technical foundations for long-term visibility",
                ],
                example_structure: vec![
                    "Corporate home",
                    "About and leadership",
                    "Services, industries, and solutions",
                    "Projects or case studies",
                    "Careers",
                    "News or resources",
                    "Advanced contact and enquiry workflows",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Multilingual functionality",
                    "Portals",
                    "API integrations",
                    "Advanced databases",
                    "Custom business systems",
                ],
                pricing_note: "Starting from LKR 250,000. Multilingual features, portals, API integrations, advanced databases, and custom systems are quoted separately.",
                faqs: vec![
                    faq(
                        "Who is The Flagship for?",
                        "It is for corporations, institutions, NGOs, large businesses, and established brands that need a substantial corporate website.",
                    ),
                    faq(
                        "Is structured data included?",
                        "Yes. Structured data is included where it is appropriate for the visible content and page types.",
                    ),
                    faq(
                        "Can this include careers and resources?",
                        "Yes. Careers, recruitment, news, insights, and resource centre sections can be included within scope.",
                    ),
                ],
                related: vec![
                    related(
                        "Custom platform",
                        "Apex Custom",
                        "/packages/web-design-development/apex-custom",
                        "Use this when the website also needs bespoke platform or workflow engineering.",
                    ),
                    related(
                        "International expansion",
                        "Global Reach",
                        "/packages/web-design-development/global-reach",
                        "Use this when country, language, and international SEO structure are central.",
                    ),
                ],
                meta_title: "Flagship Premium Corporate Website Package | LKProfessionals",
                meta_description: "The Flagship starts from LKR 250,000 for corporations, institutions, NGOs, and established brands needing a premium corporate website.",
                icon: "fa-building-columns",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "apex-custom",
                name: "Apex Custom",
                short_name: "Apex Custom",
                catalog_name: "Apex Custom - Bespoke Web Development Package",
                item_code: "LKP-WEB-AC-009",
                category: "Bespoke web engineering",
                price: "LKR 350,000+",
                price_number: "350000",
                positioning: "Bespoke web engineering where conventional websites are insufficient.",
                audience: vec![
                    "Enterprises",
                    "Organisations",
                    "Startups",
                    "Scale-ups",
                    "Businesses requiring custom platforms",
                    "Businesses requiring specialised workflows",
                ],
                core_message: "If your business can define it, we can engineer it.",
                description: "Apex Custom is for web applications, portals, dashboards, workflow systems, and bespoke platforms where a standard package is not enough.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Fully bespoke frontend and backend development",
                    "Custom UI/UX",
                    "Customer or client portals",
                    "User accounts and authentication",
                    "Role-based permissions",
                    "Custom admin dashboards",
                    "Database-driven functionality",
                    "Advanced forms and workflows",
                    "Booking or reservation systems",
                    "Payment gateways",
                    "Third-party APIs",
                    "CRM integrations",
                    "ERP integrations",
                    "Automated notifications",
                    "Reporting and analytics dashboards",
                    "Content management",
                    "Advanced search and filtering",
                    "Technical SEO foundations",
                    "On-page SEO foundations",
                    "Analytics and conversion tracking",
                    "Security/SSL",
                    "Performance optimisation",
                    "Production deployment and configuration",
                ],
                differentiator: "Individually scoped engineering for platforms, portals, dashboards, and specialised business workflows.",
                business_problem: "Your requirement is not just a website. You need custom logic, users, permissions, data, integrations, workflows, or reporting.",
                outcomes: vec![
                    "A scoped technical solution designed around real workflows",
                    "A maintainable production platform rather than a forced website template",
                    "A foundation that can connect with business systems where required",
                ],
                example_structure: vec![
                    "Discovery and technical scope",
                    "Custom frontend and backend",
                    "User roles and permissions",
                    "Data model and workflows",
                    "Admin dashboard",
                    "Integrations",
                    "Deployment and handover",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Mobile applications",
                    "Advanced DevOps",
                    "Dedicated support plans",
                    "Security reviews",
                    "Complex third-party integrations",
                ],
                pricing_note: "Starting from LKR 350,000. This is not a fixed-scope application price; every Apex Custom project is individually scoped and quoted.",
                faqs: vec![
                    faq(
                        "Is LKR 350,000 a fixed application price?",
                        "No. Apex Custom starts from LKR 350,000. Every custom platform is individually scoped and quoted.",
                    ),
                    faq(
                        "Can Apex include portals and dashboards?",
                        "Yes. Client portals, admin dashboards, user accounts, permissions, and reporting dashboards are common fit areas.",
                    ),
                    faq(
                        "When should I choose Apex Custom?",
                        "Choose it when conventional website packages cannot cover the workflow, data, integration, or platform requirements.",
                    ),
                ],
                related: vec![
                    related(
                        "Custom engineering",
                        "Discuss Scope",
                        "/request-quote",
                        "Apex Custom should be scoped through a consultation rather than treated as a normal tier upgrade.",
                    ),
                    related(
                        "SaaS or platform",
                        "Platform Forge",
                        "/packages/app-software-development/platform-forge",
                        "Use this if the custom build is really a SaaS, subscription, marketplace, or multi-user platform.",
                    ),
                    related(
                        "Enterprise system",
                        "Nexus Enterprise",
                        "/packages/app-software-development/nexus-enterprise",
                        "Use this when custom development involves complex organisational workflows and operations.",
                    ),
                ],
                meta_title: "Apex Custom Bespoke Web Development Package | LKProfessionals",
                meta_description: "Apex Custom starts from LKR 350,000 for bespoke web applications, portals, dashboards, workflows, integrations, and custom platform development.",
                icon: "fa-code-branch",
            },
            WebPackage {
                family_name: "Web Design & Development Packages",
                family_url: "/packages/web-design-development",
                family_label: "Web Design & Development",
                slug: "global-reach",
                name: "Global Reach",
                short_name: "Global Reach",
                catalog_name: "Global Reach - International & Multilingual Website Package",
                item_code: "LKP-WEB-GR-010",
                category: "International and multilingual website",
                price: "LKR 300,000+",
                price_number: "300000",
                positioning: "International and multilingual websites for businesses expanding across markets.",
                audience: vec![
                    "Exporters",
                    "International service providers",
                    "Tourism and travel companies",
                    "Education providers",
                    "Overseas recruitment businesses",
                    "Multinational organisations",
                    "Businesses expanding into new countries",
                ],
                core_message: "One website. Multiple markets. Global opportunity.",
                description: "Global Reach supports businesses expanding across countries, languages, or regions with multilingual structure, international SEO foundations, hreflang, localised CTAs, and enquiry routing.",
                hero_suffix: "web design and development package",
                delivery_label: "Responsive website build",
                inclusions: vec![
                    "Premium custom website",
                    "Multi-country architecture",
                    "Multilingual functionality",
                    "Country or regional landing pages",
                    "Language-specific content",
                    "International SEO foundation",
                    "SEO-friendly regional and language URL architecture",
                    "hreflang implementation",
                    "Location-specific services and content",
                    "Country-specific CTAs",
                    "International enquiry routing",
                    "Multi-currency presentation where required",
                    "Localised contact information",
                    "WhatsApp and social integration",
                    "Advanced CMS",
                    "Administration dashboard",
                    "Google Analytics",
                    "Google Search Console",
                    "Conversion and event tracking",
                    "Technical SEO",
                    "Performance optimisation",
                    "SSL/security",
                ],
                differentiator: "International architecture, multilingual content structure, hreflang, and market-specific conversion paths are planned together.",
                business_problem: "Your website needs to serve buyers across markets, languages, currencies, or regions without confusing users or search engines.",
                outcomes: vec![
                    "Clear country and language paths for international visitors",
                    "Search-friendly regional URL and hreflang foundations",
                    "Market-specific CTAs and enquiry routing",
                ],
                example_structure: vec![
                    "Global home",
                    "Country or region pages",
                    "Language-specific service pages",
                    "International contact routes",
                    "Market-specific CTAs",
                    "Advanced CMS",
                    "Analytics and Search Console setup",
                ],
                structure_heading: "Typical website structure or feature set.",
                process: website_process(),
                process_heading: "A clear path from requirement to launch.",
                addons: vec![
                    "Professional translation",
                    "Local market copywriting",
                    "Advanced localisation workflows",
                    "International paid landing pages",
                    "Custom portals or platforms",
                ],
                pricing_note: "Starting from LKR 300,000. Final pricing depends on number of languages, countries, pages, localisation requirements, and integrations.",
                faqs: vec![
                    faq(
                        "Does Global Reach include multiple languages?",
                        "It can include multilingual functionality. Final scope depends on the number of languages, pages, and localisation requirements.",
                    ),
                    faq(
                        "Is hreflang included?",
                        "Yes. hreflang implementation is included where multilingual or regional versions require it.",
                    ),
                    faq(
                        "When should this move to Apex Custom?",
                        "If the international website also needs complex platform features, portals, or bespoke workflows, Apex Custom may be the better direction.",
                    ),
                ],
                related: vec![related(
                    "Highly custom international platform",
                    "Apex Custom",
                    "/packages/web-design-development/apex-custom",
                    "Use this when multilingual expansion also requires bespoke platform engineering.",
                )],
                meta_title: "Global Reach Multilingual Website Package | LKProfessionals",
                meta_description: "Global Reach starts from LKR 300,000 for exporters, tourism, education, recruitment, and international businesses needing multilingual websites.",
                icon: "fa-earth-asia",
            },
        ],
    }
}

pub fn app_software_family() -> WebPackageFamily {
    macro_rules! app_pkg {
        (
            $slug:expr, $name:expr, $short:expr, $catalog:expr, $code:expr, $category:expr,
            $price:expr, $price_number:expr, $positioning:expr, $audience:expr,
            $core:expr, $description:expr, $inclusions:expr, $differentiator:expr,
            $problem:expr, $outcomes:expr, $structure:expr, $structure_heading:expr,
            $process:expr, $process_heading:expr, $addons:expr, $pricing_note:expr,
            $faqs:expr, $related:expr, $meta_title:expr, $meta_description:expr, $icon:expr
        ) => {
            WebPackage {
                family_name: "App & Software Development Packages",
                family_url: "/packages/app-software-development",
                family_label: "App & Software Development",
                slug: $slug,
                name: $name,
                short_name: $short,
                catalog_name: $catalog,
                item_code: $code,
                category: $category,
                price: $price,
                price_number: $price_number,
                positioning: $positioning,
                audience: $audience,
                core_message: $core,
                description: $description,
                hero_suffix: "app and software development package",
                delivery_label: "Scoped software engagement",
                inclusions: $inclusions,
                differentiator: $differentiator,
                business_problem: $problem,
                outcomes: $outcomes,
                example_structure: $structure,
                structure_heading: $structure_heading,
                process: $process,
                process_heading: $process_heading,
                addons: $addons,
                pricing_note: $pricing_note,
                faqs: $faqs,
                related: $related,
                meta_title: $meta_title,
                meta_description: $meta_description,
                icon: $icon,
            }
        };
    }

    WebPackageFamily {
        name: "App & Software Development Packages",
        short_name: "App & Software Development",
        url: "/packages/app-software-development",
        meta_title: "App & Software Development Packages Sri Lanka | LKProfessionals",
        meta_description: "Explore LKProfessionals app and software development packages from discovery and MVPs to mobile apps, enterprise systems, AI, SaaS platforms, scale, and modernisation.",
        schema_description: "LKProfessionals app and software development package collection for product discovery, MVP development, mobile apps, enterprise systems, AI applications, SaaS platforms, high-scale engineering, and modernisation.",
        hero_title: "Build software with a clear path from idea to scalable product.",
        hero_description: "LKProfessionals supports the full software product lifecycle: idea, discovery, validation, MVP, application development, platform development, enterprise systems, AI, scale, and modernisation.",
        whatsapp_text: "Hello%20LKProfessionals%2C%20I%20want%20help%20choosing%20an%20app%20and%20software%20development%20package.",
        how_it_works: vec![
            "These are professional software engineering engagements, not commodity products or one-size-fits-all templates.",
            "Choose by product stage, users, workflows, technical complexity, integrations, scale, and modernisation risk.",
            "Every final quote is scoped around requirements, architecture, roles, screens, integrations, deployment, and support needs.",
        ],
        range_heading: "From product discovery to enterprise software and high-scale platforms.",
        guidance_title: "Start with the product stage.",
        guidance_items: vec![
            "Still shaping the idea? Start with The Blueprint.",
            "Need a controlled first launch? Compare MVP Liftoff and Sprint App.",
            "Need iOS and Android? Review Nexus Mobile.",
            "Need connected apps, portals, dashboards, and backend systems? Review The Ecosystem.",
            "Need enterprise workflows, AI, SaaS, scale, or modernisation? Compare Nexus Enterprise, DeepTech AI, Platform Forge, OmniScale, and Next-Gen.",
        ],
        useful_links: vec![
            package_link(
                "Software Development",
                "/services/software-development",
                "See the broader engineering capability behind these packages.",
            ),
            package_link(
                "Custom Software",
                "/services/custom-software-development",
                "Useful for enterprise systems, platforms, and workflow-heavy applications.",
            ),
            package_link(
                "Mobile App Development",
                "/services/mobile-app-development",
                "Relevant for Nexus Mobile, Sprint App, and multi-platform product builds.",
            ),
            package_link(
                "Case Studies",
                "/case-studies",
                "Review selected implementation context before starting a software enquiry.",
            ),
        ],
        faqs: vec![
            faq(
                "Which app or software package should I choose?",
                "Choose by product stage. Blueprint is for discovery, MVP Liftoff is for validation, Sprint App is for focused workflows, Nexus Mobile is for iOS and Android, Ecosystem is for connected products, Enterprise is for complex operations, DeepTech AI is for purposeful AI, Platform Forge is for SaaS, OmniScale is for scale, and Next-Gen is for modernisation.",
            ),
            faq(
                "Are these fixed-price software products?",
                "No. All prices are starting prices. Final pricing depends on features, roles, screens, workflows, integrations, architecture, testing, deployment, and documentation requirements.",
            ),
            faq(
                "Can LKProfessionals help before development starts?",
                "Yes. The Blueprint package is designed for product discovery, requirements planning, architecture, wireframes, prototypes, roadmap, preliminary scope, and preliminary quotation.",
            ),
        ],
        final_cta_heading: "Need help choosing the right software path?",
        final_cta_description: "Share the product idea, user types, required platforms, must-have workflows, integrations, current system if any, and target launch timeline. LKProfessionals can recommend a package or scope a custom engagement.",
        lifecycle_steps: software_lifecycle(),
        packages: vec![
            app_pkg!(
                "the-blueprint",
                "The Blueprint",
                "Blueprint",
                "The Blueprint - Product Discovery & Prototyping Package",
                "LKP-APP-BP-001",
                "Product discovery and prototyping",
                "LKR 25,000+",
                "25000",
                "Paid product discovery, requirements planning, architecture and prototyping before production development.",
                vec![
                    "Startups",
                    "Founders",
                    "Entrepreneurs",
                    "Businesses planning custom software",
                    "Product teams",
                    "Organisations exploring digital transformation"
                ],
                "Before you build it, blueprint it.",
                "The Blueprint is a discovery and planning engagement for teams that need clearer requirements, scope, architecture, prototype direction, roadmap, and preliminary quotation before production development.",
                vec![
                    "Product discovery consultation",
                    "Business and product requirement analysis",
                    "Feature identification",
                    "Feature prioritisation",
                    "MVP scope definition",
                    "User journeys",
                    "Workflow planning",
                    "UI/UX wireframes",
                    "Key screen designs",
                    "Click-through prototype",
                    "User roles and permissions planning",
                    "Technical architecture planning",
                    "Database and data-flow planning",
                    "API and integration requirements",
                    "Technology stack recommendations",
                    "Development roadmap",
                    "Phase planning",
                    "Preliminary development scope",
                    "Preliminary quotation"
                ],
                "It reduces software-build ambiguity before larger development spend begins.",
                "You have a product idea or business workflow, but production development would be risky without a clearer scope, prototype, architecture, and phased roadmap.",
                vec![
                    "A clearer MVP or product scope",
                    "Better alignment between business goals, users, workflows, and technical approach",
                    "A more useful basis for development quotation without treating discovery as production software"
                ],
                vec![
                    "Idea and business goal",
                    "User roles and journeys",
                    "Feature priorities",
                    "Wireframes or prototype",
                    "Architecture and data-flow notes",
                    "Development roadmap and preliminary scope"
                ],
                "Discovery output and planning artefacts.",
                blueprint_process(),
                "A discovery process that ends in prototype, specification, roadmap, and scope direction.",
                vec![
                    "Additional prototype screens",
                    "User research",
                    "Technical proof of concept",
                    "Brand/UI direction",
                    "Production development phase"
                ],
                "Starting from LKR 25,000. Production software development is not automatically included; final pricing depends on complexity, screens, workflows, roles, and technical planning requirements.",
                vec![
                    faq(
                        "Does The Blueprint include production development?",
                        "No. It is a discovery and planning engagement. Production development is scoped separately after requirements are clearer."
                    ),
                    faq(
                        "What do I get at the end?",
                        "Typical outputs can include prioritised features, user journeys, workflow notes, wireframes, key screens, prototype direction, architecture notes, roadmap, and preliminary quotation."
                    ),
                    faq(
                        "Who should use this?",
                        "Founders, startups, product teams, and organisations exploring custom software or digital transformation should use this before committing to a larger build."
                    )
                ],
                vec![
                    related(
                        "Next step",
                        "MVP Liftoff",
                        "/packages/app-software-development/mvp-liftoff",
                        "Use this when the validated scope is ready for a controlled first build."
                    ),
                    related(
                        "Focused build",
                        "Sprint App",
                        "/packages/app-software-development/sprint-app",
                        "Use this when discovery reveals one focused workflow or application."
                    ),
                    related(
                        "Mobile path",
                        "Nexus Mobile",
                        "/packages/app-software-development/nexus-mobile",
                        "Use this when the validated requirement is primarily iOS and Android."
                    )
                ],
                "Blueprint Product Discovery Package | LKProfessionals",
                "The Blueprint starts from LKR 25,000 for product discovery, requirements planning, architecture, wireframes, prototype direction, roadmap, and preliminary quote.",
                "fa-pen-ruler"
            ),
            app_pkg!(
                "mvp-liftoff",
                "MVP Liftoff",
                "MVP Liftoff",
                "MVP Liftoff - Minimum Viable Product Development Package",
                "LKP-APP-MVP-002",
                "Minimum viable product development",
                "LKR 150,000+",
                "150000",
                "Build the essential first version of a digital product for real-world validation.",
                vec![
                    "Startups",
                    "Founders",
                    "Entrepreneurs",
                    "Internal product initiatives",
                    "Organisations validating software concepts"
                ],
                "Build what matters. Launch. Learn. Scale.",
                "MVP Liftoff helps teams build a controlled first product version with essential features, UI/UX implementation, backend, deployment, testing, documentation, and handover.",
                vec![
                    "MVP scope refinement",
                    "Core feature development",
                    "Responsive UI/UX implementation",
                    "Web or mobile application",
                    "Authentication",
                    "Basic roles and permissions",
                    "Database implementation",
                    "Backend/API",
                    "Administration dashboard",
                    "Essential integrations",
                    "Notifications",
                    "Analytics",
                    "Security fundamentals",
                    "Testing",
                    "Production deployment",
                    "Initial technical documentation",
                    "Handover"
                ],
                "It focuses production effort on the smallest useful product that can be tested in the real world.",
                "You need to validate a software concept without overbuilding the first release or mistaking a broad vision for a launchable product.",
                vec![
                    "A launchable first version with controlled scope",
                    "A technical foundation for learning and future phases",
                    "A development approach that treats MVP as focused, not low-quality"
                ],
                vec![
                    "Core user application",
                    "Authentication",
                    "Backend/API",
                    "Database",
                    "Admin dashboard",
                    "Essential integrations",
                    "Analytics and deployment"
                ],
                "Typical MVP product composition.",
                software_process(),
                "A production process for controlled first-version software.",
                vec![
                    "Advanced features",
                    "Native mobile apps",
                    "Complex automation",
                    "Payment/subscription systems",
                    "Post-launch product iteration"
                ],
                "Starting from LKR 150,000. Advanced functionality and subsequent phases are separately scoped after the MVP boundaries are defined.",
                vec![
                    faq(
                        "Does MVP mean low-quality software?",
                        "No. MVP means controlled scope for validation. The software should still be secure, testable, and maintainable within its agreed scope."
                    ),
                    faq(
                        "Can an MVP be web or mobile?",
                        "Yes. The first version may be a web or mobile application depending on product requirements."
                    ),
                    faq(
                        "What happens after MVP launch?",
                        "Future phases can add features, mobile apps, integrations, automation, scale improvements, or platform capabilities based on real validation."
                    )
                ],
                vec![
                    related(
                        "SaaS path",
                        "Platform Forge",
                        "/packages/app-software-development/platform-forge",
                        "Use this if the MVP is becoming a commercial SaaS or subscription platform."
                    ),
                    related(
                        "Mobile-first path",
                        "Nexus Mobile",
                        "/packages/app-software-development/nexus-mobile",
                        "Use this if mobile apps are central to adoption."
                    ),
                    related(
                        "Multi-platform path",
                        "The Ecosystem",
                        "/packages/app-software-development/the-ecosystem",
                        "Use this when the product needs apps, portals, backend, and dashboards working together."
                    )
                ],
                "MVP Liftoff Software Development Package | LKProfessionals",
                "MVP Liftoff starts from LKR 150,000 for startups and organisations building an essential first software product for validation.",
                "fa-paper-plane"
            ),
            app_pkg!(
                "nexus-mobile",
                "Nexus Mobile",
                "Nexus Mobile",
                "Nexus Mobile - iOS & Android App Development Package",
                "LKP-APP-NM-003",
                "iOS and Android app development",
                "LKR 300,000+",
                "300000",
                "Production-oriented cross-platform mobile applications.",
                vec![
                    "Startups",
                    "Service businesses",
                    "Booking platforms",
                    "Membership platforms",
                    "Customer applications",
                    "Internal applications",
                    "Organisations requiring iOS and Android"
                ],
                "One application. Two platforms. One seamless experience.",
                "Nexus Mobile is for production-oriented cross-platform mobile applications backed by custom UI/UX, authentication, backend, APIs, admin dashboard, notifications, testing, and store deployment assistance.",
                vec![
                    "iOS application",
                    "Android application",
                    "Cross-platform architecture",
                    "Custom mobile UI/UX",
                    "Authentication",
                    "User profiles",
                    "Account management",
                    "Role-based functionality",
                    "Backend/database",
                    "APIs",
                    "Administration dashboard",
                    "Push notifications",
                    "Email/in-app notifications",
                    "Media/file uploads",
                    "Search/filtering",
                    "Maps/location where required",
                    "Payments where required",
                    "Social login where required",
                    "Analytics/event tracking",
                    "Security",
                    "Performance optimisation",
                    "Device/OS testing",
                    "App Store deployment assistance",
                    "Google Play deployment assistance",
                    "Production deployment",
                    "Technical handover"
                ],
                "It brings mobile apps, backend services, admin operations, and store readiness into one scoped mobile engagement.",
                "You need mobile apps on iOS and Android, but the product also needs backend logic, accounts, notifications, dashboards, or integrations to work properly.",
                vec![
                    "A production-ready mobile application foundation",
                    "Consistent cross-platform experience for iOS and Android users",
                    "Operational backend and admin capability to support the app"
                ],
                vec![
                    "iOS app",
                    "Android app",
                    "Shared application logic",
                    "API/backend",
                    "Database",
                    "Admin dashboard",
                    "Notifications",
                    "Store deployment assistance"
                ],
                "Mobile application architecture.",
                software_process(),
                "A mobile app process from requirements to store deployment assistance.",
                vec![
                    "Native platform-specific development",
                    "Specialised hardware/device integrations",
                    "Advanced real-time systems",
                    "Large-scale infrastructure",
                    "Ongoing app maintenance"
                ],
                "Starting from LKR 300,000. Native platform-specific development or specialised hardware/device integrations are scoped separately where required.",
                vec![
                    faq(
                        "Does Nexus Mobile include both iOS and Android?",
                        "Yes. It is intended for organisations requiring iOS and Android applications."
                    ),
                    faq(
                        "Is App Store and Google Play submission included?",
                        "Deployment assistance is included, but account ownership, platform approvals, and store policies remain external factors."
                    ),
                    faq(
                        "Can the app include payments or maps?",
                        "Payments, maps, and location features can be included where required and technically scoped."
                    )
                ],
                vec![
                    related(
                        "Expanded platform",
                        "The Ecosystem",
                        "/packages/app-software-development/the-ecosystem",
                        "Use this when web, admin, backend, and integrations become substantial."
                    ),
                    related(
                        "Scale path",
                        "OmniScale",
                        "/packages/app-software-development/omniscale",
                        "Use this when traffic, concurrency, or infrastructure scale becomes central."
                    )
                ],
                "Nexus Mobile iOS & Android App Package | LKProfessionals",
                "Nexus Mobile starts from LKR 300,000 for production-oriented iOS and Android apps with backend, APIs, admin dashboard, notifications, testing, and deployment assistance.",
                "fa-mobile-screen-button"
            ),
            app_pkg!(
                "sprint-app",
                "Sprint App",
                "Sprint App",
                "Sprint App - Rapid Application Development Package",
                "LKP-APP-SA-004",
                "Rapid focused application development",
                "LKR 200,000+",
                "200000",
                "Focused application development around one clearly defined business requirement or workflow.",
                vec![
                    "Booking apps",
                    "Internal business tools",
                    "Customer portals",
                    "Membership apps",
                    "Field-service applications",
                    "Calculators",
                    "Workflow utilities",
                    "Focused digital products"
                ],
                "Focused idea. Rapid execution. Real application.",
                "Sprint App is for focused mobile or web applications built around one clearly defined workflow, such as booking, portals, field tools, calculators, memberships, or workflow utilities.",
                vec![
                    "Focused mobile/web application",
                    "Custom UI/UX",
                    "iOS/Android support where required",
                    "Authentication",
                    "User profiles",
                    "Core workflow",
                    "Backend/database",
                    "Admin dashboard",
                    "APIs/integrations",
                    "Notifications",
                    "Search/filtering",
                    "File/media handling",
                    "Analytics",
                    "Security",
                    "Performance optimisation",
                    "Testing",
                    "Deployment",
                    "App Store/Google Play assistance where applicable",
                    "Technical handover"
                ],
                "It keeps scope intentionally focused so a real application can be delivered around a defined workflow.",
                "You have one high-value workflow or application idea that needs implementation without the complexity of a larger platform programme.",
                vec![
                    "A usable app around a defined business workflow",
                    "Reduced scope drift through a focused feature set",
                    "A foundation that can expand into mobile, SaaS, or ecosystem builds later"
                ],
                vec![
                    "Focused user interface",
                    "Core workflow",
                    "Backend/database",
                    "Admin dashboard",
                    "Notifications",
                    "Integrations where required",
                    "Deployment"
                ],
                "Focused app composition.",
                software_process(),
                "A rapid application process for controlled feature sets.",
                vec![
                    "Large enterprise modules",
                    "Multi-platform suites",
                    "Advanced SaaS billing",
                    "High-scale infrastructure",
                    "Complex AI workflows"
                ],
                "Starting from LKR 200,000. Sprint App is for controlled feature sets, not a substitute for large enterprise or platform projects.",
                vec![
                    faq(
                        "Is Sprint App for large platforms?",
                        "No. Sprint App is for focused applications and controlled feature sets."
                    ),
                    faq(
                        "Can it be mobile or web?",
                        "Yes. It can be a focused mobile or web application depending on requirements."
                    ),
                    faq(
                        "Can Sprint App grow later?",
                        "Yes. It can evolve toward Nexus Mobile, The Ecosystem, or Platform Forge when the product model expands."
                    )
                ],
                vec![
                    related(
                        "Larger mobile product",
                        "Nexus Mobile",
                        "/packages/app-software-development/nexus-mobile",
                        "Use this when iOS and Android become the main product requirement."
                    ),
                    related(
                        "Connected platform",
                        "The Ecosystem",
                        "/packages/app-software-development/the-ecosystem",
                        "Use this when multiple apps, portals, dashboards, and backend systems must work together."
                    ),
                    related(
                        "SaaS model",
                        "Platform Forge",
                        "/packages/app-software-development/platform-forge",
                        "Use this if the app is becoming a subscription or multi-user platform."
                    )
                ],
                "Sprint App Rapid Application Development Package | LKProfessionals",
                "Sprint App starts from LKR 200,000 for focused mobile or web applications around booking, portals, memberships, field service, calculators, and workflows.",
                "fa-bolt"
            ),
            app_pkg!(
                "the-ecosystem",
                "The Ecosystem",
                "Ecosystem",
                "The Ecosystem - Complete Digital Product Suite",
                "LKP-APP-EC-005",
                "Complete digital product suite",
                "LKR 500,000+",
                "500000",
                "A connected suite of mobile apps, web platforms, dashboards, APIs and backend systems.",
                vec![
                    "Booking platforms",
                    "Marketplaces",
                    "Delivery businesses",
                    "Service platforms",
                    "Membership systems",
                    "Education platforms",
                    "Healthcare/business platforms",
                    "Businesses digitising customer-facing operations"
                ],
                "One business. Every platform. Working together.",
                "The Ecosystem brings together mobile apps, customer web portals, admin dashboards, APIs, backend services, data synchronisation, payments, workflows, integrations, testing, deployment, documentation, and handover.",
                vec![
                    "iOS app",
                    "Android app",
                    "Customer web application/portal",
                    "Admin dashboard",
                    "Cross-platform UI/UX",
                    "Central backend",
                    "Database",
                    "API architecture",
                    "Authentication",
                    "RBAC",
                    "Customer/staff accounts",
                    "Data synchronisation",
                    "Notifications",
                    "Payments",
                    "Booking/order/transaction workflows",
                    "File/media management",
                    "Search/filtering",
                    "Reporting/analytics",
                    "Third-party integrations",
                    "Audit/activity tracking",
                    "Security",
                    "Performance optimisation",
                    "QA/testing",
                    "Cloud/production deployment",
                    "Store deployment assistance",
                    "Documentation/handover"
                ],
                "It treats a digital product suite as one connected system rather than disconnected apps.",
                "Your business needs customer-facing applications, operational dashboards, backend logic, data, payments, and integrations to work together.",
                vec![
                    "Connected digital operations across apps, portals, dashboards, and backend",
                    "A clearer architecture for multi-platform product delivery",
                    "Operational visibility through admin, reporting, and workflow tools"
                ],
                vec![
                    "Mobile apps",
                    "API",
                    "Backend",
                    "Database",
                    "Web portal",
                    "Admin dashboard",
                    "Integrations"
                ],
                "Connected product-suite architecture.",
                software_process(),
                "A multi-platform development process from architecture to deployment.",
                vec![
                    "Advanced enterprise modules",
                    "Large-scale infrastructure",
                    "AI/ML capabilities",
                    "Multi-vendor marketplaces",
                    "Ongoing operations support"
                ],
                "Starting from LKR 500,000. Final pricing depends on platforms, roles, workflows, integrations, payments, deployment, and documentation requirements.",
                vec![
                    faq(
                        "Is The Ecosystem only for mobile apps?",
                        "No. It is for connected mobile apps, web portals, admin dashboards, APIs, backend systems, and integrations."
                    ),
                    faq(
                        "Can this include payments and bookings?",
                        "Yes. Payments, bookings, orders, or transaction workflows can be included when scoped."
                    ),
                    faq(
                        "When should this move to Enterprise or OmniScale?",
                        "Move toward Nexus Enterprise for complex organisational operations, or OmniScale when substantial traffic, concurrency, or infrastructure scale is the main challenge."
                    )
                ],
                vec![
                    related(
                        "Enterprise operations",
                        "Nexus Enterprise",
                        "/packages/app-software-development/nexus-enterprise",
                        "Use this when organisational complexity and internal operations become deeper."
                    ),
                    related(
                        "Scale engineering",
                        "OmniScale",
                        "/packages/app-software-development/omniscale",
                        "Use this when substantial traffic and infrastructure concerns dominate."
                    ),
                    related(
                        "SaaS model",
                        "Platform Forge",
                        "/packages/app-software-development/platform-forge",
                        "Use this when the product itself is a commercial SaaS or platform business."
                    )
                ],
                "Ecosystem Complete Digital Product Suite | LKProfessionals",
                "The Ecosystem starts from LKR 500,000 for connected mobile apps, web portals, admin dashboards, APIs, backend systems, workflows, and integrations.",
                "fa-network-wired"
            ),
            app_pkg!(
                "nexus-enterprise",
                "Nexus Enterprise",
                "Nexus Enterprise",
                "Nexus Enterprise - Enterprise Software Development Package",
                "LKP-APP-NE-006",
                "Enterprise software development",
                "LKR 750,000+",
                "750000",
                "Custom enterprise software for complex organisational operations.",
                vec![
                    "Corporations",
                    "Institutions",
                    "Multi-branch businesses",
                    "Manufacturers",
                    "Distributors",
                    "Large service organisations",
                    "Businesses replacing spreadsheets",
                    "Businesses replacing disconnected tools",
                    "Legacy internal systems"
                ],
                "Complex operations. One intelligent system.",
                "Nexus Enterprise is for complex organisational software with advanced workflows, RBAC, management dashboards, CRM/ERP-style modules, reporting, integrations, automation, audit trails, migration, security, testing, deployment, and handover.",
                vec![
                    "Enterprise web applications",
                    "Custom business software",
                    "Advanced backend architecture",
                    "Enterprise database design",
                    "Multi-department workflows",
                    "Advanced RBAC",
                    "Granular permissions",
                    "Management dashboards",
                    "Approval workflows",
                    "CRM functionality",
                    "ERP functionality",
                    "Finance/operations modules",
                    "Executive reporting",
                    "Document management",
                    "Search/filtering",
                    "APIs/integrations",
                    "Existing-system integration",
                    "Workflow automation",
                    "Email/SMS/system notifications",
                    "Audit trails",
                    "Activity logging",
                    "Data import/migration",
                    "Security architecture",
                    "Backup/recovery planning",
                    "Performance optimisation",
                    "Scalable deployment",
                    "QA/testing",
                    "Documentation/handover"
                ],
                "It scopes enterprise software around real operational complexity, not a generic dashboard.",
                "Your organisation has multi-department workflows, approvals, data, reporting, integrations, or legacy processes that cannot be solved by spreadsheets or disconnected tools.",
                vec![
                    "Centralised operational workflows",
                    "Better visibility for management and teams",
                    "A system architecture planned around permissions, data, reporting, integration, and long-term maintainability"
                ],
                vec![
                    "Departments and roles",
                    "Approval workflows",
                    "Enterprise database",
                    "Business modules",
                    "Dashboards and reporting",
                    "Integrations",
                    "Audit and security controls"
                ],
                "Enterprise system composition.",
                software_process(),
                "An enterprise process with deeper requirements, architecture, testing, and handover.",
                vec![
                    "AI capabilities",
                    "Advanced infrastructure scaling",
                    "Dedicated support plans",
                    "Complex data migration",
                    "Mobile companion apps"
                ],
                "Starting from LKR 750,000. Every implementation is individually scoped.",
                vec![
                    faq(
                        "Is every Nexus Enterprise project custom?",
                        "Yes. Enterprise systems must be individually scoped around operations, roles, modules, integrations, data, migration, and deployment needs."
                    ),
                    faq(
                        "Can it replace spreadsheets?",
                        "Yes, where spreadsheet-based processes are holding back visibility, workflow control, or reporting."
                    ),
                    faq(
                        "Can ERP or CRM modules be included?",
                        "ERP-style and CRM-style functionality can be included when scoped around the organisation's real workflow."
                    )
                ],
                vec![
                    related(
                        "AI capability",
                        "DeepTech AI",
                        "/packages/app-software-development/deeptech-ai",
                        "Use this when AI adds genuine business value to enterprise workflows."
                    ),
                    related(
                        "Scale engineering",
                        "OmniScale",
                        "/packages/app-software-development/omniscale",
                        "Use this where infrastructure scale becomes critical."
                    )
                ],
                "Nexus Enterprise Software Development Package | LKProfessionals",
                "Nexus Enterprise starts from LKR 750,000 for custom enterprise software, complex workflows, RBAC, dashboards, integrations, reporting, automation, and migration.",
                "fa-sitemap"
            ),
            app_pkg!(
                "deeptech-ai",
                "DeepTech AI",
                "DeepTech AI",
                "DeepTech AI - AI-Powered Application Development Package",
                "LKP-APP-AI-007",
                "AI-powered application development",
                "LKR 500,000+",
                "500000",
                "Practical AI application engineering tied to genuine business requirements.",
                vec![
                    "AI-native startups",
                    "Knowledge-intensive businesses",
                    "Document-heavy organisations",
                    "Internal knowledge systems",
                    "Customer-service systems",
                    "Businesses automating processes",
                    "AI-enabled SaaS"
                ],
                "Don't add AI for the hype. Engineer it for a purpose.",
                "DeepTech AI is for practical AI applications, assistants, RAG systems, private knowledge bases, document analysis, semantic search, workflow automation, AI APIs, backend, authentication, monitoring, validation, guardrails, security, testing, deployment, and documentation.",
                vec![
                    "AI-powered web/mobile applications",
                    "AI assistants",
                    "Intelligent interfaces",
                    "LLM integration",
                    "RAG systems",
                    "Private knowledge bases",
                    "Document analysis",
                    "Information extraction",
                    "Semantic search",
                    "NLP workflows",
                    "Content processing",
                    "Classification",
                    "Recommendations",
                    "Vision AI where appropriate",
                    "Speech/voice functionality where appropriate",
                    "AI workflow automation",
                    "Agentic workflows where appropriate",
                    "External AI APIs",
                    "Backend/database",
                    "Authentication",
                    "Administration/monitoring",
                    "Usage analytics",
                    "Output validation",
                    "Guardrails",
                    "Security/access controls",
                    "Testing",
                    "Deployment",
                    "Documentation"
                ],
                "It connects AI capability to defined business use cases, data, permissions, validation, and operational workflows.",
                "You want AI in the product or process, but it needs to solve a genuine business problem rather than become an unsupported feature demo.",
                vec![
                    "Purposeful AI features tied to user and business workflows",
                    "A safer foundation with validation, access controls, and guardrails",
                    "Practical integration with knowledge, documents, systems, or product interfaces"
                ],
                vec![
                    "Application",
                    "AI layer",
                    "Knowledge/data",
                    "Business systems",
                    "Monitoring and validation",
                    "Security/access controls"
                ],
                "AI application architecture.",
                software_process(),
                "An AI application process that validates use case, data, integration, testing, and guardrails.",
                vec![
                    "Custom model training",
                    "Fine-tuning",
                    "Third-party model/API usage costs",
                    "Large-scale AI infrastructure",
                    "Specialist data labelling"
                ],
                "Starting from LKR 500,000. Custom training/fine-tuning and third-party model or API usage costs must be assessed separately.",
                vec![
                    faq(
                        "Is custom AI model training included?",
                        "No. Custom training or fine-tuning must be assessed separately."
                    ),
                    faq(
                        "Are third-party AI API costs included?",
                        "Third-party model, API, infrastructure, and usage costs are separately scoped unless explicitly specified."
                    ),
                    faq(
                        "Can DeepTech AI support RAG or private knowledge bases?",
                        "Yes. RAG systems, private knowledge bases, document analysis, semantic search, and intelligent interfaces can be included when appropriate."
                    )
                ],
                vec![
                    related(
                        "AI SaaS",
                        "Platform Forge",
                        "/packages/app-software-development/platform-forge",
                        "Use this when AI becomes a commercial platform capability."
                    ),
                    related(
                        "Enterprise AI",
                        "Nexus Enterprise",
                        "/packages/app-software-development/nexus-enterprise",
                        "Use this when AI must sit inside broader enterprise workflows."
                    ),
                    related(
                        "High-scale AI app",
                        "OmniScale",
                        "/packages/app-software-development/omniscale",
                        "Use this when scale, concurrency, and infrastructure become central."
                    )
                ],
                "DeepTech AI Application Development Package | LKProfessionals",
                "DeepTech AI starts from LKR 500,000 for practical AI-powered applications, assistants, RAG, document analysis, semantic search, automation, guardrails, and deployment.",
                "fa-brain"
            ),
            app_pkg!(
                "omniscale",
                "OmniScale",
                "OmniScale",
                "OmniScale - High-Scale Application Development Package",
                "LKP-APP-OS-008",
                "High-scale application development",
                "LKR 1,000,000+",
                "1000000",
                "Applications engineered for substantial traffic, concurrency and growth.",
                vec![
                    "SaaS products",
                    "Marketplaces",
                    "Booking platforms",
                    "High-volume e-commerce",
                    "Consumer applications",
                    "Growing startups",
                    "Large digital platforms"
                ],
                "Built for today. Engineered for what comes next.",
                "OmniScale is a specialised application and infrastructure engineering engagement for substantial traffic, concurrency, performance, cloud-readiness, database optimisation, caching, queues, observability, CI/CD, testing, security, and deployment.",
                vec![
                    "High-scale application development",
                    "Scalable backend architecture",
                    "Cloud-ready architecture",
                    "High-performance APIs",
                    "Advanced database architecture",
                    "Database optimisation",
                    "Caching",
                    "Queues",
                    "Background workers",
                    "Load balancing architecture",
                    "Horizontal scaling readiness",
                    "CDN strategy",
                    "Asset-delivery optimisation",
                    "Real-time functionality",
                    "Distributed architecture where justified",
                    "Authentication/access control",
                    "Rate limiting",
                    "Abuse protection",
                    "Monitoring",
                    "Observability",
                    "Logging",
                    "Error tracking",
                    "CI/CD",
                    "Backup/recovery planning",
                    "Performance/load testing",
                    "Security architecture",
                    "Infrastructure documentation",
                    "Production deployment"
                ],
                "It treats scale as an engineering problem that should be justified by real application requirements.",
                "Your application needs careful architecture for substantial traffic, concurrency, background work, performance, monitoring, deployment, and growth.",
                vec![
                    "A more resilient application architecture for growth",
                    "Improved performance and observability foundations",
                    "Scale decisions based on requirements rather than unnecessary complexity"
                ],
                vec![
                    "Client applications",
                    "API/application layer",
                    "Cache/queues",
                    "Database",
                    "Infrastructure/monitoring",
                    "CI/CD and recovery planning"
                ],
                "Scale-oriented application architecture.",
                software_process(),
                "A scale-focused engineering process with architecture, load testing, observability, and deployment planning.",
                vec![
                    "Contractual SLA commitments",
                    "Dedicated SRE retainers",
                    "Complex cloud migrations",
                    "Security audits by third parties",
                    "Distributed systems where not justified"
                ],
                "Starting from LKR 1,000,000. Architecture is selected according to actual engineering requirements; no unsupported uptime guarantees are made.",
                vec![
                    faq(
                        "Does OmniScale always use microservices?",
                        "No. Distributed systems or microservices are used only where justified by engineering requirements."
                    ),
                    faq(
                        "Do you guarantee uptime?",
                        "No uptime guarantee is implied unless a separate contractual SLA supports it."
                    ),
                    faq(
                        "Who is OmniScale for?",
                        "It is for SaaS products, marketplaces, booking platforms, high-volume commerce, consumer apps, and large digital platforms where scale is a core requirement."
                    )
                ],
                vec![related(
                    "Specialised scale",
                    "Discuss Scope",
                    "/request-quote",
                    "OmniScale is a specialised scale and infrastructure engineering engagement, not simply the highest tier."
                )],
                "OmniScale High-Scale Application Development | LKProfessionals",
                "OmniScale starts from LKR 1,000,000 for high-scale application development, performance, concurrency, cloud-ready architecture, observability, testing, and deployment.",
                "fa-gauge-high"
            ),
            app_pkg!(
                "platform-forge",
                "Platform Forge",
                "Platform Forge",
                "Platform Forge - SaaS & Digital Platform Development Package",
                "LKP-APP-PF-009",
                "SaaS and digital platform development",
                "LKR 600,000+",
                "600000",
                "Commercial SaaS products, subscription platforms and multi-user digital platforms.",
                vec![
                    "SaaS startups",
                    "Subscription businesses",
                    "B2B platforms",
                    "Membership platforms",
                    "Customer portals",
                    "Marketplaces",
                    "Businesses commercialising internal processes"
                ],
                "Build the platform behind the business.",
                "Platform Forge is for SaaS and digital platforms with multi-user architecture, accounts, authentication, RBAC, subscriptions, recurring billing, customer self-service, admin dashboards, workflows, integrations, reporting, analytics, usage tracking, audit logs, testing, deployment, and handover.",
                vec![
                    "Custom SaaS development",
                    "Multi-user architecture",
                    "Multi-tenant architecture where required",
                    "Customer/organisation accounts",
                    "Authentication",
                    "RBAC",
                    "Subscription plans",
                    "Account tiers",
                    "Recurring billing",
                    "Payment gateway integration",
                    "Customer self-service portal",
                    "Administration dashboard",
                    "User/organisation management",
                    "Product/service management",
                    "Custom workflows",
                    "APIs/integrations",
                    "Notifications",
                    "Document/file management",
                    "Search/filtering",
                    "Reporting",
                    "Analytics",
                    "Usage tracking",
                    "Audit logs",
                    "Activity logs",
                    "Security/access controls",
                    "Database architecture",
                    "Performance optimisation",
                    "Testing",
                    "Deployment",
                    "Documentation/handover"
                ],
                "It builds the operational platform behind subscription, membership, marketplace, or multi-user software businesses.",
                "Your product is not just an app; it is a commercial platform with organisations, users, plans, billing, workflows, administration, analytics, and support operations.",
                vec![
                    "A platform foundation for SaaS or subscription operations",
                    "Account, billing, permissions, and admin structures that support a software business",
                    "A clearer architecture for future platform growth"
                ],
                vec![
                    "Users/organisations",
                    "Authentication/RBAC",
                    "Platform services",
                    "Subscriptions/billing",
                    "Database",
                    "Admin/analytics"
                ],
                "SaaS platform architecture.",
                software_process(),
                "A platform process covering product, architecture, billing, users, workflows, testing, and deployment.",
                vec![
                    "Third-party fees",
                    "Infrastructure costs",
                    "Advanced AI capability",
                    "High-scale infrastructure programme",
                    "Dedicated support and operations"
                ],
                "Starting from LKR 600,000. Third-party fees and infrastructure costs are separately scoped where applicable.",
                vec![
                    faq(
                        "Can Platform Forge include subscriptions?",
                        "Yes. Subscription plans, account tiers, recurring billing, and payment gateway integration can be included when scoped."
                    ),
                    faq(
                        "Is multi-tenancy always required?",
                        "No. Multi-tenant architecture is used where the platform model requires it."
                    ),
                    faq(
                        "Can an internal process become a SaaS product?",
                        "Yes. Platform Forge can support businesses commercialising internal processes as a customer-facing platform."
                    )
                ],
                vec![
                    related(
                        "AI platform capability",
                        "DeepTech AI",
                        "/packages/app-software-development/deeptech-ai",
                        "Use this when AI becomes a core platform capability."
                    ),
                    related(
                        "Scale path",
                        "OmniScale",
                        "/packages/app-software-development/omniscale",
                        "Use this as the platform reaches significant scale."
                    )
                ],
                "Platform Forge SaaS Development Package | LKProfessionals",
                "Platform Forge starts from LKR 600,000 for SaaS products, subscription platforms, multi-user digital platforms, accounts, billing, dashboards, workflows, and analytics.",
                "fa-layer-group"
            ),
            app_pkg!(
                "next-gen",
                "The Next-Gen Upgrade",
                "Next-Gen Upgrade",
                "The Next-Gen Upgrade - Application Modernisation Package",
                "LKP-APP-NGU-010",
                "Application modernisation",
                "LKR 400,000+",
                "400000",
                "Modernisation and major evolution of existing applications and legacy software.",
                vec![
                    "Legacy systems",
                    "Existing business applications",
                    "Outdated SaaS products",
                    "Older mobile applications",
                    "Systems requiring major new features",
                    "Applications outgrowing their existing stack"
                ],
                "Don't let yesterday's technology limit tomorrow's business.",
                "The Next-Gen Upgrade is for assessing, modernising, refactoring, redesigning, migrating, and evolving existing applications, legacy systems, SaaS products, or older mobile applications.",
                vec![
                    "Technical assessment",
                    "Legacy codebase review",
                    "Architecture review",
                    "UI/UX redesign",
                    "Frontend modernisation",
                    "Backend modernisation/refactoring",
                    "Database optimisation",
                    "Database restructuring",
                    "Performance improvements",
                    "Mobile responsiveness",
                    "API modernisation",
                    "Integration upgrades",
                    "Authentication/permissions modernisation",
                    "Security improvements",
                    "Major feature development",
                    "Admin dashboard upgrades",
                    "Reporting/analytics improvements",
                    "Workflow redesign",
                    "Automation",
                    "Cloud migration where appropriate",
                    "Infrastructure modernisation",
                    "Data migration",
                    "Deployment automation",
                    "Testing/QA",
                    "Production migration planning",
                    "Documentation/handover"
                ],
                "It starts with assessment so modernisation, migration, refactoring, or rebuild decisions are commercially and technically grounded.",
                "Your existing application is limiting the business through outdated technology, performance issues, poor UX, weak security, missing features, or difficult maintenance.",
                vec![
                    "A safer path for evolving existing software",
                    "Modernisation strategy based on assessment rather than assumptions",
                    "Improved UX, architecture, performance, security, or feature capability where scoped"
                ],
                vec![
                    "Existing application",
                    "Assessment",
                    "Modernisation strategy",
                    "Migration plan",
                    "Development",
                    "Testing/QA",
                    "Controlled deployment"
                ],
                "Modernisation path.",
                next_gen_process(),
                "A modernisation process that starts with assessment and controlled migration planning.",
                vec![
                    "Full rebuild",
                    "Major data migration",
                    "Cloud infrastructure programme",
                    "Security audit",
                    "Ongoing maintenance"
                ],
                "Starting from LKR 400,000. Existing applications must be assessed before final scope confirmation; rebuilding may be recommended if it is safer than upgrading in place.",
                vec![
                    faq(
                        "Can every legacy system be upgraded in place?",
                        "No. Some systems are safer or more cost-effective to rebuild. Assessment determines the recommended path."
                    ),
                    faq(
                        "Does this include assessment?",
                        "Yes. Technical assessment, codebase review, architecture review, and modernisation strategy can be part of the engagement."
                    ),
                    faq(
                        "Why is the route /next-gen?",
                        "The public route is intentionally shortened to /packages/app-software-development/next-gen."
                    )
                ],
                vec![
                    related(
                        "Enterprise rebuild",
                        "Nexus Enterprise",
                        "/packages/app-software-development/nexus-enterprise",
                        "Use this if an internal legacy system requires a major enterprise rebuild."
                    ),
                    related(
                        "SaaS redevelopment",
                        "Platform Forge",
                        "/packages/app-software-development/platform-forge",
                        "Use this if an existing SaaS product requires substantial platform redevelopment."
                    ),
                    related(
                        "Scale problem",
                        "OmniScale",
                        "/packages/app-software-development/omniscale",
                        "Use this when scalability is the primary modernisation problem."
                    )
                ],
                "Next-Gen Application Modernisation Package | LKProfessionals",
                "The Next-Gen Upgrade starts from LKR 400,000 for legacy application assessment, modernisation, refactoring, migration planning, feature upgrades, testing, and controlled deployment.",
                "fa-arrows-rotate"
            ),
        ],
    }
}

pub fn find_web_package(slug: &str) -> Option<WebPackage> {
    web_design_family()
        .packages
        .into_iter()
        .find(|package| package.slug == slug)
}

pub fn find_app_software_package(slug: &str) -> Option<WebPackage> {
    app_software_family()
        .packages
        .into_iter()
        .find(|package| package.slug == slug)
}

fn faq(question: &'static str, answer: &'static str) -> PackageFaq {
    PackageFaq { question, answer }
}

fn package_link(title: &'static str, href: &'static str, note: &'static str) -> PackageLink {
    PackageLink { title, href, note }
}

fn step(title: &'static str, description: &'static str) -> PackageStep {
    PackageStep { title, description }
}

fn website_process() -> Vec<PackageStep> {
    vec![
        step(
            "Discovery",
            "Clarify goals, users, constraints, content, and commercial priorities.",
        ),
        step(
            "Planning",
            "Define pages, features, information architecture, and scope boundaries.",
        ),
        step(
            "Design",
            "Create responsive layouts aligned with brand, buyers, and content.",
        ),
        step(
            "Development",
            "Build the site, forms, CMS, integrations, SEO foundations, and tracking.",
        ),
        step(
            "Review",
            "Test content, responsiveness, forms, performance, and important user paths.",
        ),
        step(
            "Launch",
            "Deploy securely, connect essentials, and hand over the agreed setup.",
        ),
    ]
}

fn software_lifecycle() -> Vec<PackageStep> {
    vec![
        step(
            "Idea",
            "Clarify the business opportunity and product direction.",
        ),
        step(
            "Discovery",
            "Turn uncertainty into requirements, workflows, and scope.",
        ),
        step(
            "Validation",
            "Prioritise what should be tested or launched first.",
        ),
        step(
            "MVP",
            "Build the essential first version where appropriate.",
        ),
        step(
            "Application Development",
            "Implement web, mobile, backend, database, and integrations.",
        ),
        step(
            "Platform Development",
            "Shape multi-user, SaaS, portal, dashboard, and API capability.",
        ),
        step(
            "Enterprise Systems",
            "Engineer deeper operations, permissions, reporting, and workflows.",
        ),
        step(
            "AI",
            "Add AI only where it solves a real product or business requirement.",
        ),
        step(
            "Scale",
            "Improve performance, observability, infrastructure, and growth readiness.",
        ),
        step(
            "Modernisation",
            "Evolve existing applications safely through assessment and controlled change.",
        ),
    ]
}

fn software_process() -> Vec<PackageStep> {
    vec![
        step(
            "Discovery",
            "Clarify users, goals, workflows, constraints, and success criteria.",
        ),
        step(
            "Requirements",
            "Define features, roles, data, integrations, and scope boundaries.",
        ),
        step(
            "Architecture",
            "Plan application structure, backend, database, APIs, security, and deployment.",
        ),
        step(
            "UI/UX",
            "Design practical interfaces for users, admins, and operational workflows.",
        ),
        step(
            "Development",
            "Build the agreed application, platform, integrations, and management tools.",
        ),
        step(
            "Testing",
            "Validate core flows, security fundamentals, responsiveness, performance, and edge cases.",
        ),
        step(
            "Deployment",
            "Release to the agreed production environment or stores where applicable.",
        ),
        step(
            "Handover",
            "Provide documentation, access, and technical handover for the scoped system.",
        ),
    ]
}

fn blueprint_process() -> Vec<PackageStep> {
    vec![
        step(
            "Discovery",
            "Understand the idea, business model, users, constraints, and priorities.",
        ),
        step(
            "Requirements",
            "Capture workflows, roles, features, data, and integration needs.",
        ),
        step(
            "Prioritisation",
            "Separate must-have MVP scope from later-phase possibilities.",
        ),
        step(
            "Wireframes",
            "Map key screens, journeys, and interface direction.",
        ),
        step(
            "Prototype",
            "Prepare click-through or screen-level prototype direction where scoped.",
        ),
        step(
            "Architecture",
            "Outline technical approach, data flow, integrations, and stack direction.",
        ),
        step(
            "Roadmap",
            "Define phases, development scope, and preliminary quotation direction.",
        ),
    ]
}

fn next_gen_process() -> Vec<PackageStep> {
    vec![
        step(
            "Assessment",
            "Review the existing application, codebase, architecture, UX, data, and risk.",
        ),
        step(
            "Modernisation Strategy",
            "Decide whether to refactor, modernise, migrate, rebuild, or phase the work.",
        ),
        step(
            "Migration Plan",
            "Plan data, deployment, rollout, integrations, and continuity requirements.",
        ),
        step(
            "Development",
            "Implement approved modernisation, redesign, refactoring, or feature work.",
        ),
        step(
            "Testing",
            "Validate critical workflows, data integrity, performance, and security fundamentals.",
        ),
        step(
            "Controlled Deployment",
            "Release changes with a managed rollout and handover plan.",
        ),
    ]
}

fn related(
    label: &'static str,
    name: &'static str,
    href: &'static str,
    note: &'static str,
) -> RelatedPackage {
    RelatedPackage {
        label,
        name,
        href,
        note,
    }
}
