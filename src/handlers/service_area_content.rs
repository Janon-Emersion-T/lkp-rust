use super::{
    service_content::{ServiceCard, all_service_cards},
    templates::{
        ServiceAreaCardView, ServiceAreaDetailFaqView, ServiceAreaDetailPointView,
        ServiceAreaGroupView, ServiceAreaPageView,
    },
};

#[derive(Clone, Copy)]
struct ServiceAreaRecord {
    slug: &'static str,
    city: &'static str,
    country: &'static str,
    region: &'static str,
    market_angle: &'static str,
    timezone_note: &'static str,
    nearby_markets: &'static [&'static str],
}

#[derive(Clone, Copy)]
pub struct ServiceAreaSitemapEntry {
    pub title: &'static str,
    pub path: &'static str,
    pub description: &'static str,
}

const SERVICE_AREA_GROUPS: [(&str, &str); 5] = [
    (
        "Sri Lanka",
        "Local SEO and business software positioning for the markets closest to LKProfessionals.",
    ),
    (
        "UK & Europe",
        "Commercial markets looking for offshore development support, SEO execution, and scalable delivery partners.",
    ),
    (
        "North America",
        "Software, web, and growth support for companies that need a capable remote engineering partner.",
    ),
    (
        "Middle East",
        "Fast-moving business markets where software delivery, automation, and digital infrastructure matter commercially.",
    ),
    (
        "Asia-Pacific",
        "High-standard markets where clarity, speed, and operational reliability are expected from delivery partners.",
    ),
];

const SERVICE_AREAS: [ServiceAreaRecord; 22] = [
    ServiceAreaRecord {
        slug: "jaffna",
        city: "Jaffna",
        country: "Sri Lanka",
        region: "Sri Lanka",
        market_angle: "custom software development, business websites, SEO, and automation support",
        timezone_note: "Fast local coordination with direct access to the LKProfessionals team.",
        nearby_markets: &["Colombo", "Kandy", "Batticaloa"],
    },
    ServiceAreaRecord {
        slug: "colombo",
        city: "Colombo",
        country: "Sri Lanka",
        region: "Sri Lanka",
        market_angle: "software delivery, enterprise websites, SEO growth, and digital transformation",
        timezone_note: "Strong same-country collaboration for teams that need responsiveness and execution discipline.",
        nearby_markets: &["Jaffna", "Galle", "Kandy"],
    },
    ServiceAreaRecord {
        slug: "london",
        city: "London",
        country: "United Kingdom",
        region: "UK & Europe",
        market_angle: "offshore software development, web platforms, SEO execution, and long-term technical support",
        timezone_note: "Working overlap suitable for UK teams that want structured offshore delivery without losing momentum.",
        nearby_markets: &["Manchester", "Dublin", "Amsterdam"],
    },
    ServiceAreaRecord {
        slug: "manchester",
        city: "Manchester",
        country: "United Kingdom",
        region: "UK & Europe",
        market_angle: "custom software builds, lead-generation websites, and growth-ready technical support",
        timezone_note: "Useful overlap for agencies, service firms, and scaling businesses working with remote teams.",
        nearby_markets: &["London", "Dublin", "Berlin"],
    },
    ServiceAreaRecord {
        slug: "dublin",
        city: "Dublin",
        country: "Ireland",
        region: "UK & Europe",
        market_angle: "software product support, business systems, and SEO-friendly digital platforms",
        timezone_note: "Remote delivery built around predictable communication and commercially practical scope control.",
        nearby_markets: &["London", "Amsterdam", "Berlin"],
    },
    ServiceAreaRecord {
        slug: "berlin",
        city: "Berlin",
        country: "Germany",
        region: "UK & Europe",
        market_angle: "software engineering, product interfaces, automation, and offshore delivery support",
        timezone_note: "European-friendly collaboration model for product teams and service businesses.",
        nearby_markets: &["Amsterdam", "Paris", "London"],
    },
    ServiceAreaRecord {
        slug: "amsterdam",
        city: "Amsterdam",
        country: "Netherlands",
        region: "UK & Europe",
        market_angle: "web applications, software platforms, SEO systems, and structured offshore execution",
        timezone_note: "Well suited for companies that need quality engineering support without inflated local overhead.",
        nearby_markets: &["Berlin", "Paris", "London"],
    },
    ServiceAreaRecord {
        slug: "paris",
        city: "Paris",
        country: "France",
        region: "UK & Europe",
        market_angle: "business platforms, conversion-focused websites, and scalable digital delivery",
        timezone_note: "Remote collaboration with enough overlap for planning, reviews, and delivery checkpoints.",
        nearby_markets: &["Amsterdam", "Berlin", "Dublin"],
    },
    ServiceAreaRecord {
        slug: "new-york",
        city: "New York",
        country: "United States",
        region: "North America",
        market_angle: "offshore product development, custom platforms, SEO, and business automation",
        timezone_note: "Built for asynchronous execution with clear updates, documented scope, and reliable handoff.",
        nearby_markets: &["Toronto", "Chicago", "Austin"],
    },
    ServiceAreaRecord {
        slug: "san-francisco",
        city: "San Francisco",
        country: "United States",
        region: "North America",
        market_angle: "product engineering support, MVP execution, web platforms, and automation systems",
        timezone_note: "Useful for startup and growth teams working with remote engineering capacity across time zones.",
        nearby_markets: &["Los Angeles", "Austin", "Vancouver"],
    },
    ServiceAreaRecord {
        slug: "chicago",
        city: "Chicago",
        country: "United States",
        region: "North America",
        market_angle: "business systems, enterprise web delivery, SEO support, and automation builds",
        timezone_note: "Strong fit for businesses that care about clear scope, delivery discipline, and maintainability.",
        nearby_markets: &["New York", "Toronto", "Austin"],
    },
    ServiceAreaRecord {
        slug: "austin",
        city: "Austin",
        country: "United States",
        region: "North America",
        market_angle: "startup product support, web software, SEO execution, and technical delivery capacity",
        timezone_note: "Remote model suited for teams that prefer lean execution and practical iteration.",
        nearby_markets: &["San Francisco", "Chicago", "Toronto"],
    },
    ServiceAreaRecord {
        slug: "toronto",
        city: "Toronto",
        country: "Canada",
        region: "North America",
        market_angle: "custom software, digital growth systems, and offshore development support",
        timezone_note: "Good fit for Canadian companies that want capable remote delivery with accountable communication.",
        nearby_markets: &["Vancouver", "New York", "Chicago"],
    },
    ServiceAreaRecord {
        slug: "vancouver",
        city: "Vancouver",
        country: "Canada",
        region: "North America",
        market_angle: "web platforms, software engineering, automation, and digital operations support",
        timezone_note: "Asynchronous-first collaboration suited for product teams and service businesses.",
        nearby_markets: &["Toronto", "San Francisco", "Auckland"],
    },
    ServiceAreaRecord {
        slug: "dubai",
        city: "Dubai",
        country: "United Arab Emirates",
        region: "Middle East",
        market_angle: "custom software, corporate websites, SEO, and automation for growth-focused companies",
        timezone_note: "Strong overlap for discovery, project reviews, and ongoing execution.",
        nearby_markets: &["Abu Dhabi", "Doha", "Riyadh"],
    },
    ServiceAreaRecord {
        slug: "abu-dhabi",
        city: "Abu Dhabi",
        country: "United Arab Emirates",
        region: "Middle East",
        market_angle: "business platforms, secure delivery, and long-term technology support",
        timezone_note: "Timezone compatibility supports close communication and faster issue resolution.",
        nearby_markets: &["Dubai", "Doha", "Riyadh"],
    },
    ServiceAreaRecord {
        slug: "doha",
        city: "Doha",
        country: "Qatar",
        region: "Middle East",
        market_angle: "enterprise websites, software systems, and digital transformation delivery",
        timezone_note: "Good alignment for businesses that want offshore capability with regional accessibility.",
        nearby_markets: &["Dubai", "Abu Dhabi", "Riyadh"],
    },
    ServiceAreaRecord {
        slug: "riyadh",
        city: "Riyadh",
        country: "Saudi Arabia",
        region: "Middle East",
        market_angle: "business software, automation, cloud-backed systems, and structured delivery",
        timezone_note: "Built for practical collaboration with regional businesses and international-facing teams.",
        nearby_markets: &["Dubai", "Doha", "Abu Dhabi"],
    },
    ServiceAreaRecord {
        slug: "singapore",
        city: "Singapore",
        country: "Singapore",
        region: "Asia-Pacific",
        market_angle: "high-standard web software, technical SEO, automation, and business system execution",
        timezone_note: "Strong fit for teams expecting structured communication and technically clean delivery.",
        nearby_markets: &["Sydney", "Melbourne", "Auckland"],
    },
    ServiceAreaRecord {
        slug: "sydney",
        city: "Sydney",
        country: "Australia",
        region: "Asia-Pacific",
        market_angle: "custom software, conversion-focused websites, SEO, and offshore development support",
        timezone_note: "Convenient operating overlap for Australian companies working with Sri Lankan delivery teams.",
        nearby_markets: &["Melbourne", "Singapore", "Auckland"],
    },
    ServiceAreaRecord {
        slug: "melbourne",
        city: "Melbourne",
        country: "Australia",
        region: "Asia-Pacific",
        market_angle: "web platforms, software systems, automation, and digital growth support",
        timezone_note: "Strong overlap for project coordination, QA cycles, and launch planning.",
        nearby_markets: &["Sydney", "Singapore", "Auckland"],
    },
    ServiceAreaRecord {
        slug: "auckland",
        city: "Auckland",
        country: "New Zealand",
        region: "Asia-Pacific",
        market_angle: "software delivery, digital operations support, and offshore execution capacity",
        timezone_note: "Useful for teams that want organized asynchronous delivery with dependable checkpoints.",
        nearby_markets: &["Sydney", "Melbourne", "Singapore"],
    },
];

pub fn service_area_groups() -> Vec<ServiceAreaGroupView> {
    SERVICE_AREA_GROUPS
        .iter()
        .map(|(title, description)| {
            let areas = SERVICE_AREAS
                .iter()
                .filter(|area| area.region == *title)
                .map(to_card_view)
                .collect::<Vec<_>>();

            ServiceAreaGroupView {
                title: (*title).to_string(),
                description: (*description).to_string(),
                areas,
            }
        })
        .collect()
}

pub fn all_service_area_cards() -> Vec<ServiceAreaCardView> {
    SERVICE_AREAS.iter().map(to_card_view).collect()
}

pub fn service_area_count() -> usize {
    SERVICE_AREAS.len()
}

pub fn service_area_featured_services() -> Vec<ServiceCard> {
    all_service_cards()
        .into_iter()
        .filter(|service| {
            matches!(
                service.slug,
                "/services/custom-software-development"
                    | "/services/web-development"
                    | "/services/seo-search-growth"
                    | "/services/ai-automation-solutions"
            )
        })
        .collect()
}

pub fn service_area_page(slug: &str) -> Option<ServiceAreaPageView> {
    let area = SERVICE_AREAS.iter().find(|area| area.slug == slug)?;

    Some(ServiceAreaPageView {
        city: area.city.to_string(),
        region: area.region.to_string(),
        title: format!("{}, {}", area.city, area.country),
        canonical_path: format!("/service-areas/{}", area.slug),
        meta_title: format!(
            "Software Development Company in {} | LKProfessionals",
            area.city
        ),
        meta_description: format!(
            "LKProfessionals serves businesses in {}, {} with custom software development, web applications, SEO, automation, and offshore delivery support from Sri Lanka.",
            area.city, area.country
        ),
        hero_title: format!(
            "Software development and digital growth support for businesses in {}.",
            area.city
        ),
        hero_description: format!(
            "LKProfessionals works with companies in {}, {} that need {}. We combine technical delivery, conversion thinking, and operational clarity without the cost structure of a large-city agency.",
            area.city, area.country, area.market_angle
        ),
        positioning: format!(
            "LKProfessionals is headquartered in Jaffna, Sri Lanka and serves clients in {} through structured offshore delivery, direct communication, and practical scope control.",
            area.city
        ),
        timezone_note: area.timezone_note.to_string(),
        nearby_markets: area.nearby_markets.iter().map(|market| (*market).to_string()).collect(),
        buyer_points: vec![
            ServiceAreaDetailPointView {
                title: format!("Offshore delivery for {}", area.city),
                description: format!(
                    "A practical partner for businesses in {} that want capable engineering support, commercial clarity, and lower delivery overhead than many local-market options.",
                    area.city
                ),
            },
            ServiceAreaDetailPointView {
                title: "Good fit for service businesses and growth teams".to_string(),
                description: "Useful for SMEs, startups, agencies, and established companies that need websites, business systems, SEO support, or automation.".to_string(),
            },
            ServiceAreaDetailPointView {
                title: "Execution built around real business priorities".to_string(),
                description: "The work focuses on launch quality, maintainability, speed, and lead-generation impact instead of decorative output only.".to_string(),
            },
        ],
        service_points: vec![
            ServiceAreaDetailPointView {
                title: "Custom software and internal systems".to_string(),
                description: format!(
                    "Business software, dashboards, workflow tools, and operational systems for teams in {} that need better process control.",
                    area.city
                ),
            },
            ServiceAreaDetailPointView {
                title: "Web platforms and conversion-focused websites".to_string(),
                description: "Corporate websites, landing pages, portals, and web apps designed to support trust, search visibility, and lead capture.".to_string(),
            },
            ServiceAreaDetailPointView {
                title: "SEO, automation, and digital operations".to_string(),
                description: "Technical SEO, growth content structure, AI-assisted workflows, and automation support for commercial teams.".to_string(),
            },
        ],
        faqs: vec![
            ServiceAreaDetailFaqView {
                question: format!("Does LKProfessionals work with businesses in {}?", area.city),
                answer: format!(
                    "Yes. LKProfessionals works with businesses in {}, {} through remote discovery, structured delivery, and ongoing support.",
                    area.city, area.country
                ),
            },
            ServiceAreaDetailFaqView {
                question: format!("Why hire a Sri Lankan software development company for {} projects?", area.city),
                answer: "For many companies, the value is a stronger cost-to-capability ratio, direct communication, and a partner that can combine software delivery, SEO, automation, and business thinking in one team.".to_string(),
            },
            ServiceAreaDetailFaqView {
                question: "What services can be delivered remotely?".to_string(),
                answer: "Custom software development, web application development, mobile apps, SEO, digital marketing support, automation, hosting guidance, and ongoing optimization can all be delivered remotely.".to_string(),
            },
            ServiceAreaDetailFaqView {
                question: format!("Is LKProfessionals suitable for SMEs and enterprise buyers in {}?", area.city),
                answer: "Yes. The delivery model suits both smaller companies that need focused execution and larger organizations that need dependable external capacity.".to_string(),
            },
        ],
    })
}

pub fn related_service_areas(slug: &str) -> Vec<ServiceAreaCardView> {
    let Some(current) = SERVICE_AREAS.iter().find(|area| area.slug == slug) else {
        return Vec::new();
    };

    let mut same_region = SERVICE_AREAS
        .iter()
        .filter(|area| area.region == current.region && area.slug != current.slug)
        .map(to_card_view)
        .collect::<Vec<_>>();

    let mut others = SERVICE_AREAS
        .iter()
        .filter(|area| area.region != current.region)
        .map(to_card_view)
        .collect::<Vec<_>>();

    same_region.append(&mut others);
    same_region.into_iter().take(4).collect()
}

pub fn service_area_sitemap_entries() -> Vec<ServiceAreaSitemapEntry> {
    let mut entries = Vec::with_capacity(SERVICE_AREAS.len() + 1);
    entries.push(ServiceAreaSitemapEntry {
        title: "Service Areas",
        path: "/service-areas",
        description: "Global service area hub for LKProfessionals delivery markets.",
    });

    for area in SERVICE_AREAS {
        entries.push(ServiceAreaSitemapEntry {
            title: area.city,
            path: match area.slug {
                "jaffna" => "/service-areas/jaffna",
                "colombo" => "/service-areas/colombo",
                "london" => "/service-areas/london",
                "manchester" => "/service-areas/manchester",
                "dublin" => "/service-areas/dublin",
                "berlin" => "/service-areas/berlin",
                "amsterdam" => "/service-areas/amsterdam",
                "paris" => "/service-areas/paris",
                "new-york" => "/service-areas/new-york",
                "san-francisco" => "/service-areas/san-francisco",
                "chicago" => "/service-areas/chicago",
                "austin" => "/service-areas/austin",
                "toronto" => "/service-areas/toronto",
                "vancouver" => "/service-areas/vancouver",
                "dubai" => "/service-areas/dubai",
                "abu-dhabi" => "/service-areas/abu-dhabi",
                "doha" => "/service-areas/doha",
                "riyadh" => "/service-areas/riyadh",
                "singapore" => "/service-areas/singapore",
                "sydney" => "/service-areas/sydney",
                "melbourne" => "/service-areas/melbourne",
                "auckland" => "/service-areas/auckland",
                _ => "/service-areas",
            },
            description: "Service area page for LKProfessionals delivery support.",
        });
    }

    entries
}

fn to_card_view(area: &ServiceAreaRecord) -> ServiceAreaCardView {
    ServiceAreaCardView {
        region: area.region.to_string(),
        title: format!("{}, {}", area.city, area.country),
        path: format!("/service-areas/{}", area.slug),
        summary: format!(
            "LKProfessionals supports businesses in {} with {}.",
            area.city, area.market_angle
        ),
    }
}
