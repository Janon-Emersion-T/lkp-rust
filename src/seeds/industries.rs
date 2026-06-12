use sqlx::PgPool;

pub async fn seed_default_industries(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM industries
        "#,
    )
    .fetch_one(pool)
    .await?;

    if count > 0 {
        println!("Industries already seeded.");
        return Ok(());
    }

    let industries = vec![
        (
            "Retail & Commerce",
            "retail-commerce",
            "POS systems, inventory platforms, e-commerce workflows, and operational visibility for growing retail businesses.",
            "Retail businesses need tight control over products, billing, fulfillment, promotions, and customer experience across both physical and digital channels.",
            Some(
                "Disconnected sales channels and stock visibility can slow teams down and reduce margins.",
            ),
            Some(
                "We design systems that connect POS, inventory, reporting, and customer-facing experiences into one practical operating flow.",
            ),
            "fa-solid fa-store",
            true,
            1,
        ),
        (
            "Education",
            "education",
            "Learning platforms, student portals, academic workflows, and digital operations for educational institutions.",
            "Education teams need systems that support enrollments, attendance, timetables, assessments, communication, and learner experience without adding admin friction.",
            Some(
                "Manual academic administration and fragmented communication create bottlenecks for both staff and students.",
            ),
            Some(
                "We build digital systems that improve academic visibility, automate repetitive workflows, and create better access for learners.",
            ),
            "fa-solid fa-graduation-cap",
            true,
            2,
        ),
        (
            "Healthcare",
            "healthcare",
            "Patient systems, clinic operations, appointment workflows, and secure digital healthcare experiences.",
            "Healthcare providers need reliable systems for patient handling, scheduling, records, compliance-sensitive workflows, and service efficiency.",
            Some(
                "Care delivery is easily slowed by paper-heavy handling, booking confusion, and weak digital access.",
            ),
            Some(
                "We create healthcare workflows that support patient service, staff coordination, and better operational clarity.",
            ),
            "fa-solid fa-hospital",
            true,
            3,
        ),
        (
            "Corporate & Professional Services",
            "corporate-professional-services",
            "Dashboards, workflow platforms, reporting systems, and process automation for service-led organizations.",
            "Corporate and service businesses need systems that give leadership more visibility, reduce manual coordination, and support internal execution quality.",
            Some(
                "Teams often rely on scattered tools, duplicated data entry, and slow internal approvals.",
            ),
            Some(
                "We build internal systems and automation layers that make operational work clearer, faster, and easier to manage.",
            ),
            "fa-solid fa-building",
            true,
            4,
        ),
        (
            "Hospitality & Tourism",
            "hospitality-tourism",
            "Booking platforms, guest-facing websites, reservation handling, and digital engagement for hospitality brands.",
            "Hospitality businesses need digital systems that help them attract attention, manage reservations smoothly, and improve guest experience from discovery to service.",
            Some(
                "Weak booking flows and dated websites can cost both direct reservations and trust.",
            ),
            Some(
                "We design booking-ready digital experiences that support conversion, operations, and guest communication.",
            ),
            "fa-solid fa-hotel",
            false,
            5,
        ),
        (
            "Construction & Field Operations",
            "construction-field-operations",
            "Project tracking, quote systems, workforce coordination, and operational software for execution-heavy teams.",
            "Construction and field-service organizations need clearer oversight across teams, tasks, approvals, materials, and client-facing project updates.",
            Some(
                "Field-heavy work becomes difficult to manage when updates, costing, and coordination depend on scattered manual communication.",
            ),
            Some(
                "We help centralize project visibility and simplify the workflows that keep execution moving.",
            ),
            "fa-solid fa-helmet-safety",
            false,
            6,
        ),
        (
            "Logistics & Distribution",
            "logistics-distribution",
            "Fleet systems, dispatch workflows, warehouse visibility, and reporting automation for logistics operations.",
            "Logistics teams rely on timely coordination, status visibility, and repeatable workflows across inventory movement, transport, and reporting.",
            Some(
                "Manual dispatch and fragmented warehouse tracking create delays and make performance hard to measure.",
            ),
            Some(
                "We create operational systems that improve tracking, coordination, and decision-making across movement-heavy businesses.",
            ),
            "fa-solid fa-truck-fast",
            false,
            7,
        ),
        (
            "Government & NGOs",
            "government-ngos",
            "Digitization platforms, service portals, and public-facing systems for institutions and mission-driven organizations.",
            "Public and nonprofit organizations need systems that improve access, streamline administration, and support reliable information handling.",
            Some(
                "Legacy processes can make public service slower, less transparent, and harder to scale.",
            ),
            Some(
                "We help digitize workflows in ways that improve access, coordination, and service delivery quality.",
            ),
            "fa-solid fa-landmark",
            false,
            8,
        ),
    ];

    for industry in industries {
        sqlx::query(
            r#"
            INSERT INTO industries
            (
                title,
                slug,
                short_description,
                overview,
                challenge_focus,
                solution_focus,
                icon_class,
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
            ($1, $2, $3, $4, $5, $6, $7, $8, TRUE, $9, $10, $11, 'https://lkprofessionals.com/industries', $12, NOW())
            "#,
        )
        .bind(industry.0)
        .bind(industry.1)
        .bind(industry.2)
        .bind(industry.3)
        .bind(industry.4)
        .bind(industry.5)
        .bind(industry.6)
        .bind(industry.7)
        .bind(industry.8)
        .bind(format!("{} Solutions | LKProfessionals", industry.0))
        .bind(industry.2)
        .bind(match industry.1 {
            "retail-commerce" => "https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?auto=format&fit=crop&w=1200&q=80",
            "education" => "https://images.unsplash.com/photo-1522202176988-66273c2fd55f?auto=format&fit=crop&w=1200&q=80",
            "healthcare" => "https://images.unsplash.com/photo-1576091160399-112ba8d25d1d?auto=format&fit=crop&w=1200&q=80",
            "corporate-professional-services" => "https://images.unsplash.com/photo-1521737604893-d14cc237f11d?auto=format&fit=crop&w=1200&q=80",
            "hospitality-tourism" => "https://images.unsplash.com/photo-1566073771259-6a8506099945?auto=format&fit=crop&w=1200&q=80",
            "construction-field-operations" => "https://images.unsplash.com/photo-1504307651254-35680f356dfd?auto=format&fit=crop&w=1200&q=80",
            "logistics-distribution" => "https://images.unsplash.com/photo-1586528116311-ad8dd3c8310d?auto=format&fit=crop&w=1200&q=80",
            "government-ngos" => "https://images.unsplash.com/photo-1520607162513-77705c0f0d4a?auto=format&fit=crop&w=1200&q=80",
            _ => "https://images.unsplash.com/photo-1498050108023-c5249f4df085?auto=format&fit=crop&w=1200&q=80",
        })
        .execute(pool)
        .await?;
    }

    println!("Default industries seeded successfully.");

    Ok(())
}
