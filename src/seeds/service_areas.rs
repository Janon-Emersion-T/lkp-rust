use sqlx::PgPool;

struct SeedServiceArea {
    area_name: &'static str,
    slug: &'static str,
    area_type: &'static str,
    country: &'static str,
    market_region: &'static str,
    focus: &'static str,
    buyer_profile: &'static str,
    nearby_markets: &'static str,
    featured: bool,
}

pub async fn seed_default_service_areas(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM service_areas
        "#,
    )
    .fetch_one(pool)
    .await?;

    if count > 0 {
        println!("Service areas already seeded.");
        return Ok(());
    }

    let areas = vec![
        SeedServiceArea {
            area_name: "Jaffna",
            slug: "jaffna",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "custom software, websites, SEO, and automation support",
            buyer_profile: "Ideal for businesses that want a serious local-rooted technology partner with direct access to the LKProfessionals team.",
            nearby_markets: "Colombo,Kandy,Mannar",
            featured: true,
        },
        SeedServiceArea {
            area_name: "Negombo",
            slug: "negombo",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "web platforms, booking-ready websites, and growth support",
            buyer_profile: "Useful for hospitality, retail, and service businesses that need stronger digital systems and clearer lead capture.",
            nearby_markets: "Colombo,Kandy,Galle",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Kandy",
            slug: "kandy",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "business software, websites, and operational automation",
            buyer_profile: "Useful for education, healthcare, retail, and service-led businesses that need maintainable digital infrastructure.",
            nearby_markets: "Colombo,Peradeniya,Hatton",
            featured: true,
        },
        SeedServiceArea {
            area_name: "Galle",
            slug: "galle",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "hospitality websites, service platforms, and digital growth systems",
            buyer_profile: "Strong fit for tourism, hospitality, and service businesses that need modern, conversion-focused digital assets.",
            nearby_markets: "Colombo,Negombo,Kandy",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Trincomalee",
            slug: "trincomalee",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "business websites, operational systems, and structured SEO support",
            buyer_profile: "Good fit for regional businesses that need stronger visibility and practical technical delivery.",
            nearby_markets: "Batticaloa,Anuradhapura,Colombo",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Batticaloa",
            slug: "batticaloa",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "business systems, local SEO, and scalable web delivery",
            buyer_profile: "Useful for organizations that need clearer operations, better digital reach, and long-term support.",
            nearby_markets: "Trincomalee,Mullaitivu,Jaffna",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Anuradhapura",
            slug: "anuradhapura",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "web development, software delivery, and digital modernization support",
            buyer_profile: "Relevant for growing organizations that want cleaner systems and more defensible digital infrastructure.",
            nearby_markets: "Kurunegala,Colombo,Trincomalee",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Kurunegala",
            slug: "kurunegala",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "commercial websites, internal tools, and automation workflows",
            buyer_profile: "Good fit for SMEs and operational businesses that need practical implementation without unnecessary complexity.",
            nearby_markets: "Colombo,Kandy,Anuradhapura",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Vavuniya",
            slug: "vavuniya",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "business systems, regional SEO, and workflow improvement",
            buyer_profile: "Useful for organizations that want a stronger digital foundation and structured technical support.",
            nearby_markets: "Jaffna,Kilinochchi,Mannar",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Mannar",
            slug: "mannar",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "web presence, lead capture systems, and practical digital operations support",
            buyer_profile: "A fit for service businesses and regional organizations that need reliable execution and visibility.",
            nearby_markets: "Jaffna,Vavuniya,Kilinochchi",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Kilinochchi",
            slug: "kilinochchi",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "software support, websites, and digital process improvement",
            buyer_profile: "Useful for teams that need direct collaboration with a Sri Lanka-based delivery company.",
            nearby_markets: "Jaffna,Vavuniya,Mullaitivu",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Hatton",
            slug: "hatton",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "regional business websites, operational systems, and digital growth support",
            buyer_profile: "Useful for businesses that want a more serious digital presence with dependable long-term support.",
            nearby_markets: "Kandy,Peradeniya,Colombo",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Peradeniya",
            slug: "peradeniya",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "institutional platforms, business systems, and modern web delivery",
            buyer_profile: "Strong fit for education-linked, healthcare-linked, and service organizations that need maintainable digital systems.",
            nearby_markets: "Kandy,Hatton,Colombo",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Mullaitivu",
            slug: "mullaitivu",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "digital modernization, websites, and workflow support",
            buyer_profile: "Useful for organizations that want structured delivery and a clearer digital operating model.",
            nearby_markets: "Kilinochchi,Vavuniya,Batticaloa",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Colombo",
            slug: "colombo",
            area_type: "city",
            country: "Sri Lanka",
            market_region: "Sri Lanka",
            focus: "enterprise websites, software platforms, SEO systems, and automation support",
            buyer_profile: "Well suited for serious businesses comparing local agency pricing against a more efficient delivery partner.",
            nearby_markets: "Negombo,Kandy,Galle",
            featured: true,
        },
        SeedServiceArea {
            area_name: "London",
            slug: "london",
            area_type: "city",
            country: "United Kingdom",
            market_region: "UK & Europe",
            focus: "offshore software development, web platforms, and SEO execution",
            buyer_profile: "Built for companies in London that want structured offshore delivery without losing commercial clarity or pace.",
            nearby_markets: "Manchester,Glasgow,Edinburgh",
            featured: true,
        },
        SeedServiceArea {
            area_name: "Manchester",
            slug: "manchester",
            area_type: "city",
            country: "United Kingdom",
            market_region: "UK & Europe",
            focus: "custom software, lead-generation websites, and automation support",
            buyer_profile: "Strong fit for agencies, SMEs, and service businesses that need a practical remote delivery partner.",
            nearby_markets: "London,Glasgow,Edinburgh",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Glasgow",
            slug: "glasgow",
            area_type: "city",
            country: "United Kingdom",
            market_region: "UK & Europe",
            focus: "business software, technical SEO, and digital delivery support",
            buyer_profile: "Useful for teams that need clean implementation, dependable communication, and commercially grounded scoping.",
            nearby_markets: "Edinburgh,Manchester,London",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Edinburgh",
            slug: "edinburgh",
            area_type: "city",
            country: "United Kingdom",
            market_region: "UK & Europe",
            focus: "software delivery, website modernization, and automation systems",
            buyer_profile: "A fit for organizations that want a capable external team for software, SEO, and digital operations support.",
            nearby_markets: "Glasgow,Manchester,London",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Mumbai",
            slug: "mumbai",
            area_type: "city",
            country: "India",
            market_region: "India & South Asia",
            focus: "software engineering, product builds, and scalable web delivery",
            buyer_profile: "Useful for companies comparing high local delivery costs with an offshore partner that can still operate at a serious standard.",
            nearby_markets: "Tamil Nadu,Kerala,Bangladesh",
            featured: true,
        },
        SeedServiceArea {
            area_name: "Tamil Nadu",
            slug: "tamil-nadu",
            area_type: "state",
            country: "India",
            market_region: "India & South Asia",
            focus: "software development, business websites, SEO systems, and operational tooling",
            buyer_profile: "Relevant for businesses across Tamil Nadu that need direct execution support from a nearby regional delivery team.",
            nearby_markets: "Mumbai,Kerala,Jaffna",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Kerala",
            slug: "kerala",
            area_type: "state",
            country: "India",
            market_region: "India & South Asia",
            focus: "digital platforms, process automation, and SEO-ready web systems",
            buyer_profile: "Strong fit for service-led and operations-heavy businesses that need better digital execution and support.",
            nearby_markets: "Tamil Nadu,Mumbai,Dubai",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Bangladesh",
            slug: "bangladesh",
            area_type: "country",
            country: "Bangladesh",
            market_region: "India & South Asia",
            focus: "custom software, business systems, and digital modernization support",
            buyer_profile: "Useful for organizations that need a technically capable regional partner without the overhead of large onshore delivery teams.",
            nearby_markets: "India,Pakistan,Singapore",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Pakistan",
            slug: "pakistan",
            area_type: "country",
            country: "Pakistan",
            market_region: "India & South Asia",
            focus: "software delivery, websites, automation, and structured technical support",
            buyer_profile: "Relevant for businesses that need reliable implementation, better process control, and clear communication.",
            nearby_markets: "India,Bangladesh,Dubai",
            featured: false,
        },
        SeedServiceArea {
            area_name: "Dubai",
            slug: "dubai",
            area_type: "city",
            country: "United Arab Emirates",
            market_region: "Middle East",
            focus: "custom software, corporate websites, and automation delivery",
            buyer_profile: "Useful for growth-focused businesses that need fast-moving execution and a partner comfortable with international commercial expectations.",
            nearby_markets: "Abu Dhabi,Doha,Riyadh",
            featured: true,
        },
        SeedServiceArea {
            area_name: "Singapore",
            slug: "singapore",
            area_type: "city",
            country: "Singapore",
            market_region: "Asia-Pacific",
            focus: "high-standard web software, technical SEO, and automation support",
            buyer_profile: "Built for buyers who expect clean technical delivery, clarity, and commercially sensible execution.",
            nearby_markets: "Sydney,Toronto,Mumbai",
            featured: true,
        },
        SeedServiceArea {
            area_name: "Sydney",
            slug: "sydney",
            area_type: "city",
            country: "Australia",
            market_region: "Asia-Pacific",
            focus: "web platforms, custom software, SEO systems, and offshore delivery support",
            buyer_profile: "A strong fit for Australian businesses that want timezone-friendly collaboration with Sri Lankan execution capability.",
            nearby_markets: "Melbourne,Singapore,Auckland",
            featured: true,
        },
        SeedServiceArea {
            area_name: "Toronto",
            slug: "toronto",
            area_type: "city",
            country: "Canada",
            market_region: "North America",
            focus: "custom software development, business systems, and structured offshore support",
            buyer_profile: "Relevant for companies comparing expensive local builds against a more efficient but still serious delivery partner.",
            nearby_markets: "New York,Chicago,Vancouver",
            featured: true,
        },
        SeedServiceArea {
            area_name: "New York",
            slug: "new-york",
            area_type: "city",
            country: "United States",
            market_region: "North America",
            focus: "product engineering, websites, SEO support, and business automation",
            buyer_profile: "Good fit for fast-moving companies that need capable execution with clear asynchronous communication.",
            nearby_markets: "Toronto,Chicago,Austin",
            featured: true,
        },
    ];

    for (index, area) in areas.into_iter().enumerate() {
        let images = image_set(index);
        let overview = format!(
            "LKProfessionals supports businesses in {} through {}. The focus is on practical delivery: cleaner scope, dependable communication, maintainable implementation, and digital systems that can support real commercial growth. This page exists to position LKProfessionals clearly for buyers in {} who are evaluating software development companies, web partners, SEO support, automation providers, or broader digital transformation capacity.",
            area.area_name, area.focus, area.area_name
        );
        let delivery_focus = format!(
            "For {} clients, LKProfessionals combines Sri Lanka-based delivery efficiency with direct communication, structured project handling, and support across software, websites, SEO, and automation.",
            area.area_name
        );
        let timezone_note = match area.market_region {
            "Sri Lanka" => "Direct local collaboration with fast response cycles and easy coordination.".to_string(),
            "UK & Europe" => "Suitable working overlap for discovery, reviews, and milestone-based delivery without unnecessary friction.".to_string(),
            "India & South Asia" => "Regional proximity helps with communication speed, collaboration comfort, and practical delivery cadence.".to_string(),
            "Middle East" => "Strong timezone compatibility for businesses that expect timely updates and commercially clear execution.".to_string(),
            "Asia-Pacific" => "Good overlap for planning, QA cycles, and structured delivery checkpoints across time zones.".to_string(),
            _ => "Built for structured remote delivery with clear reporting and practical overlap where needed.".to_string(),
        };

        sqlx::query(
            r#"
            INSERT INTO service_areas
            (
                area_name, slug, area_type, country, market_region, short_description, overview,
                buyer_profile, delivery_focus, timezone_note, nearby_markets, hero_image_url,
                gallery_image_url_2, gallery_image_url_3, featured, published, sort_order,
                meta_title, meta_description, canonical_url, og_image_url, published_at
            )
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, TRUE, $16, $17, $18, $19, $20, NOW())
            "#,
        )
        .bind(area.area_name)
        .bind(area.slug)
        .bind(area.area_type)
        .bind(area.country)
        .bind(area.market_region)
        .bind(format!(
            "LKProfessionals supports businesses in {}, {} with {}.",
            area.area_name, area.country, area.focus
        ))
        .bind(overview)
        .bind(area.buyer_profile)
        .bind(delivery_focus)
        .bind(timezone_note)
        .bind(area.nearby_markets)
        .bind(images.0)
        .bind(images.1)
        .bind(images.2)
        .bind(area.featured)
        .bind((index as i32) + 1)
        .bind(format!(
            "Software Development in {} | LKProfessionals",
            area.area_name
        ))
        .bind(format!(
            "LKProfessionals serves {} with software development, web platforms, SEO, automation, and digital delivery support from Sri Lanka.",
            area.area_name
        ))
        .bind(format!("https://lkprofessionals.com/service-areas/{}", area.slug))
        .bind(images.0)
        .execute(pool)
        .await?;
    }

    println!("Default service areas seeded successfully.");
    Ok(())
}

fn image_set(index: usize) -> (&'static str, &'static str, &'static str) {
    const SETS: [(&str, &str, &str); 6] = [
        (
            "https://images.unsplash.com/photo-1497366754035-f200968a6e72?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1497366412874-3415097a27e7?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1520607162513-77705c0f0d4a?auto=format&fit=crop&w=1400&q=80",
        ),
        (
            "https://images.unsplash.com/photo-1486406146926-c627a92ad1ab?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1460317442991-0ec209397118?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1514565131-fce0801e5785?auto=format&fit=crop&w=1400&q=80",
        ),
        (
            "https://images.unsplash.com/photo-1517048676732-d65bc937f952?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1521737604893-d14cc237f11d?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1552664730-d307ca884978?auto=format&fit=crop&w=1400&q=80",
        ),
        (
            "https://images.unsplash.com/photo-1477959858617-67f85cf4f1df?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1449824913935-59a10b8d2000?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1431540015161-0bf868a2d407?auto=format&fit=crop&w=1400&q=80",
        ),
        (
            "https://images.unsplash.com/photo-1500530855697-b586d89ba3ee?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1512453979798-5ea266f8880c?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1504384308090-c894fdcc538d?auto=format&fit=crop&w=1400&q=80",
        ),
        (
            "https://images.unsplash.com/photo-1494526585095-c41746248156?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1449824913935-59a10b8d2000?auto=format&fit=crop&w=1400&q=80",
            "https://images.unsplash.com/photo-1470004914212-05527e49370b?auto=format&fit=crop&w=1400&q=80",
        ),
    ];

    SETS[index % SETS.len()]
}
