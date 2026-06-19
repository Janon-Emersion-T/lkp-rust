use chrono::{Duration, Utc};
use sqlx::PgPool;

type SeedResult<T> = Result<T, sqlx::Error>;

struct SeedPortfolio<'a> {
    title: &'a str,
    slug: &'a str,
    client_name: &'a str,
    industry: &'a str,
    service_category: &'a str,
    excerpt: &'a str,
    overview: &'a str,
    challenge: &'a str,
    solution: &'a str,
    results: &'a str,
    impact_metrics: &'a str,
    technologies: &'a str,
    cover_image_url: &'a str,
    live_url: &'a str,
    featured: bool,
    sort_order: i32,
}

const DEFAULT_PORTFOLIOS: &[SeedPortfolio] = &[
    SeedPortfolio {
        title: "JDIC Dental Implant Centre",
        slug: "jdic-dental-implant-centre",
        client_name: "Jaffna Dental Implant Centre",
        industry: "Healthcare",
        service_category: "Website Development + SEO",
        excerpt: "A high-trust dental implant website built to support patient inquiries, treatment discovery, and ongoing SEO growth for a specialist clinic in Jaffna.",
        overview: "We delivered a modern web presence for Jaffna Dental Implant Centre focused on clarity, trust, and treatment-led discovery. The site positions the clinic around core implant services, patient reassurance, and consultation intent while giving the brand a stronger structure for ongoing SEO work. Since this client also engages us for SEO, the build was shaped to support content expansion, service-page targeting, and local search visibility from the start.",
        challenge: "The clinic needed a more credible, conversion-oriented digital presence for specialist dental treatments, with content structure strong enough to support both patient decision-making and long-term organic visibility.",
        solution: "We built a polished healthcare website with clear service architecture, treatment-first messaging, trust indicators, and consultation-focused calls to action. We also structured the platform for SEO delivery with indexable service pages, better content hierarchy, and room for future authority-building content.",
        results: "Stronger brand positioning for specialist dental implant services\nCleaner treatment discovery for prospective patients\nImproved consultation journey through clearer calls to action\nBetter foundation for local SEO and service-led organic growth",
        impact_metrics: "Healthcare trust-led UX\nService page SEO structure\nLocal search readiness\nConsultation-focused conversion flow",
        technologies: "Responsive Web Design, Technical SEO, Service Page Architecture, Conversion Copywriting",
        cover_image_url: "https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?auto=format&fit=crop&w=1400&q=80",
        live_url: "https://jdentalimplantcentre.com/",
        featured: true,
        sort_order: 1,
    },
    SeedPortfolio {
        title: "JSF Medical",
        slug: "jsf-medical",
        client_name: "JSF Medical",
        industry: "Healthcare",
        service_category: "Website Development + SEO",
        excerpt: "A clinic website and growth-ready platform concept built around maritime medical certification workflows for seafarers and agencies.",
        overview: "JSF Medical required a clean, dependable online presence for medical certificates related to seafarer readiness. We developed a website that introduces the clinic clearly, explains the service journey, and supports appointment-focused inquiries. The delivery was also structured with SEO in mind, making it easier to expand the site's visibility around maritime medical services and certificate-related intent over time.",
        challenge: "The business needed to present a niche medical service in a way that felt trustworthy, organized, and easy to understand for seafarers, agencies, and employers who often work with time-sensitive documentation.",
        solution: "We designed a focused clinic website with clear service explanations, booking pathways, and messaging tailored to maritime medical compliance. The information architecture was planned to support both present-day patient communication and future SEO-driven landing page expansion.",
        results: "Clearer communication around maritime medical certification services\nA more structured inquiry path for appointments and documentation support\nImproved digital credibility for a specialist clinic audience\nA scalable foundation for future content and SEO growth",
        impact_metrics: "Niche healthcare positioning\nAppointment-first UX\nSearch-ready page hierarchy\nFuture internal workflow alignment",
        technologies: "Responsive Web Design, Information Architecture, Technical SEO, Content Structuring",
        cover_image_url: "https://images.unsplash.com/photo-1584515933487-779824d29309?auto=format&fit=crop&w=1400&q=80",
        live_url: "https://jsfmedical.com/",
        featured: true,
        sort_order: 2,
    },
    SeedPortfolio {
        title: "TNK Property Services",
        slug: "tnk-property-services",
        client_name: "TNK Property Services",
        industry: "Property Services",
        service_category: "Website Development + SEO",
        excerpt: "A property services website built to convert commercial and residential inquiries across Sydney while supporting long-tail SEO around service areas and maintenance categories.",
        overview: "For TNK Property Services, we created a service-led website that makes a multi-offer business easier to understand and easier to contact. The platform highlights building management, cleaning, strata maintenance, and garden care in a clear structure while supporting localized expansion through service-area and blog content. Because we also manage SEO for this client, the build was planned around scalable search visibility and quote-generation goals.",
        challenge: "The client needed a stronger digital platform to present multiple property services under one brand without confusing visitors or weakening local search relevance.",
        solution: "We structured the site around core service categories, quote-focused calls to action, and service-area expansion. The content framework supports both conversion and SEO by making each offer easier to index, understand, and extend across Sydney-focused landing pages.",
        results: "Better separation of service lines for faster user understanding\nClearer quote-generation paths across the site\nStronger local landing page structure for ongoing SEO work\nA more scalable content base for long-term service visibility",
        impact_metrics: "Multi-service architecture\nLocal SEO expansion model\nQuote-focused navigation\nService-area content readiness",
        technologies: "Responsive Web Design, Local SEO, Landing Page Strategy, Content Architecture",
        cover_image_url: "https://images.unsplash.com/photo-1504307651254-35680f356dfd?auto=format&fit=crop&w=1400&q=80",
        live_url: "https://tnkpropertyservices.com.au/",
        featured: true,
        sort_order: 3,
    },
    SeedPortfolio {
        title: "The Parking Deals",
        slug: "the-parking-deals",
        client_name: "The Parking Deals",
        industry: "Travel & Transportation",
        service_category: "Booking Platform + SEO",
        excerpt: "A UK airport parking booking platform designed for price comparison, faster bookings, and ongoing SEO growth around airports, routes, and parking types.",
        overview: "The Parking Deals was developed as a focused airport parking platform for UK travellers who need a fast quote-to-booking experience. We shaped the site around booking clarity, airport-based navigation, and service-category discovery so users can move from search to booking with minimal friction. Since we also support SEO for this client, the structure was built to strengthen visibility across airport parking intent, informational content, and transactional pages.",
        challenge: "The platform needed to balance conversion speed with search discoverability across a competitive travel niche where users expect instant clarity, trust, and pricing relevance.",
        solution: "We delivered a booking-led interface with clear airport coverage, comparison-focused messaging, and service segmentation for parking types. The page structure also supports SEO by aligning key landing pages with airport and service intent rather than relying on a single generic homepage.",
        results: "A clearer booking journey for airport parking customers\nStronger category and airport-level content structure\nBetter support for transactional and informational SEO intent\nA scalable base for travel-focused search growth",
        impact_metrics: "Booking-first UX\nAirport landing page model\nTravel intent SEO structure\nConversion-oriented page hierarchy",
        technologies: "Booking UX, Technical SEO, Landing Page Strategy, Responsive Web Design",
        cover_image_url: "https://images.unsplash.com/photo-1436491865332-7a61a109cc05?auto=format&fit=crop&w=1400&q=80",
        live_url: "https://theparkingdeals.co.uk/",
        featured: false,
        sort_order: 4,
    },
    SeedPortfolio {
        title: "Best Park Deal",
        slug: "best-park-deal",
        client_name: "Best Park Deal",
        industry: "Travel & Transportation",
        service_category: "Booking Platform + SEO",
        excerpt: "A conversion-led airport parking website for UK travellers, structured to support search visibility, price comparison journeys, and fast quote requests.",
        overview: "Best Park Deal is a travel-focused web platform built to help users compare airport parking options and book with less friction. We developed the site around a simple quote flow, visible trust cues, and clearer service discovery across airports and parking types. Because this client also works with us on SEO, the platform was designed to support category growth, landing page expansion, and long-term organic acquisition.",
        challenge: "The brand needed a cleaner booking experience and a stronger content structure for a competitive search environment where trust, speed, and destination relevance directly affect conversions.",
        solution: "We created a booking-oriented website with prominent search inputs, airport-specific pathways, and service content that can scale. The SEO-ready information architecture makes it easier to build authority around airport parking searches while keeping the user journey direct.",
        results: "Faster path from landing to quote request\nMore structured airport and service discovery\nImproved readiness for ongoing SEO campaigns\nA clearer value proposition for price-conscious travellers",
        impact_metrics: "Quote-driven interface\nTravel category SEO support\nAirport content scalability\nTrust-focused conversion design",
        technologies: "Responsive Web Design, Technical SEO, Conversion UX, Search-Landing Strategy",
        cover_image_url: "https://images.unsplash.com/photo-1494515843206-f3117d3f51b7?auto=format&fit=crop&w=1400&q=80",
        live_url: "https://bestparkdeal.co.uk/",
        featured: false,
        sort_order: 5,
    },
    SeedPortfolio {
        title: "Puthuyugam",
        slug: "puthuyugam",
        client_name: "Puthuyugam",
        industry: "Media & Publishing",
        service_category: "News Platform Development",
        excerpt: "A Tamil digital publishing platform built for high-volume content presentation, category discovery, and a cleaner reading experience across entertainment and news topics.",
        overview: "Puthuyugam is a content-heavy publishing platform serving Tamil-language readers across entertainment and news themes. We developed the website to better handle editorial volume, homepage scanning, article discovery, and category-led browsing while keeping the experience accessible on both desktop and mobile. The delivery focused on making a busy publishing environment feel more structured and easier to navigate.",
        challenge: "The project required a layout capable of supporting frequent content updates and broad topic coverage without overwhelming readers or weakening article discovery.",
        solution: "We built a media-friendly content experience with stronger hierarchy, clearer section organization, and a homepage flow suited to continuous publishing. The structure helps surface top stories, improve browseability, and support editorial growth over time.",
        results: "Improved content presentation for a high-frequency publishing workflow\nClearer category-based discovery across news and entertainment topics\nA more structured homepage experience for repeat readers\nBetter support for mobile content consumption",
        impact_metrics: "Editorial content hierarchy\nCategory-led navigation\nMobile-first publishing UX\nScalable article discovery",
        technologies: "Publishing UX, Responsive Web Design, Content Architecture, Editorial Layout Systems",
        cover_image_url: "https://images.unsplash.com/photo-1495020689067-958852a7765e?auto=format&fit=crop&w=1400&q=80",
        live_url: "https://puthuyugam.com/",
        featured: false,
        sort_order: 6,
    },
    SeedPortfolio {
        title: "Medicon International Qualification Board UK",
        slug: "medicon-international-qualification-board-uk",
        client_name: "Medicon International Qualification Board UK",
        industry: "Education",
        service_category: "Institutional Website Development",
        excerpt: "An institutional website developed to present educational credibility, qualification pathways, and trust signals for an international certification-focused organization.",
        overview: "We delivered a professional web presence for Medicon International Qualification Board UK focused on educational positioning, institutional trust, and program communication. The site was designed to help the organization present its mission, qualification-related messaging, and credibility more clearly to prospective learners and stakeholders. The result is a cleaner institutional platform that supports authority and discoverability.",
        challenge: "The organization needed a website that felt more credible and internationally aligned while clearly presenting its educational purpose and qualification-related messaging.",
        solution: "We created a structured institutional website with stronger page hierarchy, clearer communication blocks, and a presentation style suited to an education and certification audience. The delivery emphasized trust, readability, and professional positioning.",
        results: "Stronger institutional presentation for an education-focused brand\nClearer communication of qualification and organizational information\nImproved trust signals for prospective learners and partners\nA more maintainable structure for future content expansion",
        impact_metrics: "Institutional credibility design\nEducation-focused information architecture\nProfessional content hierarchy\nTrust-led brand presentation",
        technologies: "Responsive Web Design, Information Architecture, Content Structuring, Institutional UX",
        cover_image_url: "https://images.unsplash.com/photo-1523240795612-9a054b0db644?auto=format&fit=crop&w=1400&q=80",
        live_url: "https://mcniqbuk.co.uk/",
        featured: false,
        sort_order: 7,
    },
    SeedPortfolio {
        title: "CricAlertz",
        slug: "cricalertz",
        client_name: "CricAlertz",
        industry: "Sports Media",
        service_category: "Publishing Platform + SEO",
        excerpt: "A cricket news publishing platform developed for frequent updates, category-led content discovery, and SEO support across match, player, and tournament topics.",
        overview: "CricAlertz was built as a sports publishing platform focused on cricket news, player content, schedules, and tournament coverage. We shaped the site to support high-content velocity, repeat visits, and better discoverability across topic clusters that matter in cricket search. Since this client also works with us on SEO, the platform was structured to support ongoing organic growth through category pages, article indexing, and search-friendly content architecture.",
        challenge: "The site needed to publish large volumes of sports content in a way that remained organized, easy to browse, and ready for long-tail SEO across players, teams, and competitions.",
        solution: "We developed a content-rich media platform with stronger taxonomy, clearer article surfacing, and a structure suitable for both readers and search engines. The implementation supports topical depth without sacrificing navigation clarity.",
        results: "Better organization for cricket news and feature content\nImproved topic clustering across players, teams, and tournaments\nA stronger base for ongoing SEO publishing\nA more scalable publishing setup for editorial growth",
        impact_metrics: "Sports content taxonomy\nSEO-ready category architecture\nEditorial scalability\nRepeat-visit content discovery",
        technologies: "Publishing UX, Technical SEO, Taxonomy Design, Responsive Web Design",
        cover_image_url: "https://images.unsplash.com/photo-1540747913346-19e32dc3e97e?auto=format&fit=crop&w=1400&q=80",
        live_url: "http://cricalertz.com/",
        featured: false,
        sort_order: 8,
    },
];

pub async fn seed_default_portfolios(pool: &PgPool) -> SeedResult<()> {
    for (index, portfolio) in DEFAULT_PORTFOLIOS.iter().enumerate() {
        sqlx::query(
            r#"
            INSERT INTO portfolios
            (
                title,
                slug,
                client_name,
                industry,
                service_category,
                excerpt,
                overview,
                challenge,
                solution,
                results,
                impact_metrics,
                technologies,
                cover_image_url,
                live_url,
                featured,
                published,
                sort_order,
                meta_title,
                meta_description,
                canonical_url,
                og_image_url,
                published_at
            )
            VALUES
            (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, TRUE, $16,
                $17, $18, $19, $20, $21
            )
            ON CONFLICT (slug) DO UPDATE SET
                title = EXCLUDED.title,
                client_name = EXCLUDED.client_name,
                industry = EXCLUDED.industry,
                service_category = EXCLUDED.service_category,
                excerpt = EXCLUDED.excerpt,
                overview = EXCLUDED.overview,
                challenge = EXCLUDED.challenge,
                solution = EXCLUDED.solution,
                results = EXCLUDED.results,
                impact_metrics = EXCLUDED.impact_metrics,
                technologies = EXCLUDED.technologies,
                cover_image_url = EXCLUDED.cover_image_url,
                live_url = EXCLUDED.live_url,
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
        .bind(portfolio.title)
        .bind(portfolio.slug)
        .bind(portfolio.client_name)
        .bind(portfolio.industry)
        .bind(portfolio.service_category)
        .bind(portfolio.excerpt)
        .bind(portfolio.overview)
        .bind(portfolio.challenge)
        .bind(portfolio.solution)
        .bind(portfolio.results)
        .bind(portfolio.impact_metrics)
        .bind(portfolio.technologies)
        .bind(portfolio.cover_image_url)
        .bind(portfolio.live_url)
        .bind(portfolio.featured)
        .bind(portfolio.sort_order)
        .bind(format!("{} Case Study | LKProfessionals", portfolio.title))
        .bind(portfolio.excerpt)
        .bind(format!(
            "https://lkprofessionals.com/case-studies/{}",
            portfolio.slug
        ))
        .bind(portfolio.cover_image_url)
        .bind(Utc::now() - Duration::days(index as i64))
        .execute(pool)
        .await?;
    }

    println!("Default portfolios seeded successfully.");

    Ok(())
}
