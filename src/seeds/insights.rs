use std::error::Error;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;
use sqlx::PgPool;

type SeedResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const INSIGHTS_JSON: &str = include_str!("data/insights.json");

#[derive(Debug, Deserialize)]
struct SeedInsight {
    title: String,
    slug: String,
    excerpt: String,
    content: String,
    author: String,
    category: String,
    cover_image_url: String,
    featured: bool,
    published: bool,
    reading_time_minutes: i32,
    meta_title: String,
    meta_description: String,
    canonical_url: String,
    og_image_url: String,
    published_at: String,
}

pub async fn seed_default_insights(pool: &PgPool) -> SeedResult<()> {
    let insights: Vec<SeedInsight> = serde_json::from_str(INSIGHTS_JSON)?;
    let slugs: Vec<&str> = insights.iter().map(|item| item.slug.as_str()).collect();

    let existing_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM insights
        WHERE slug = ANY($1)
        "#,
    )
    .bind(&slugs)
    .fetch_one(pool)
    .await?;

    if existing_count == insights.len() as i64 {
        println!("Insights already seeded.");
        return Ok(());
    }

    for (index, insight) in insights.into_iter().enumerate() {
        let published_at = parse_published_at(&insight.published_at)?;
        let view_count = estimated_view_count(index, insight.featured);

        sqlx::query(
            r#"
            INSERT INTO insights
            (
                title,
                slug,
                excerpt,
                content,
                author,
                category,
                cover_image_url,
                featured,
                published,
                view_count,
                reading_time_minutes,
                meta_title,
                meta_description,
                canonical_url,
                og_image_url,
                published_at
            )
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            ON CONFLICT (slug) DO UPDATE SET
                title = EXCLUDED.title,
                excerpt = EXCLUDED.excerpt,
                content = EXCLUDED.content,
                author = EXCLUDED.author,
                category = EXCLUDED.category,
                cover_image_url = EXCLUDED.cover_image_url,
                featured = EXCLUDED.featured,
                published = EXCLUDED.published,
                reading_time_minutes = EXCLUDED.reading_time_minutes,
                meta_title = EXCLUDED.meta_title,
                meta_description = EXCLUDED.meta_description,
                canonical_url = EXCLUDED.canonical_url,
                og_image_url = EXCLUDED.og_image_url,
                published_at = EXCLUDED.published_at,
                updated_at = NOW()
            "#,
        )
        .bind(&insight.title)
        .bind(&insight.slug)
        .bind(&insight.excerpt)
        .bind(&insight.content)
        .bind(&insight.author)
        .bind(&insight.category)
        .bind(&insight.cover_image_url)
        .bind(insight.featured)
        .bind(insight.published)
        .bind(view_count)
        .bind(insight.reading_time_minutes)
        .bind(&insight.meta_title)
        .bind(&insight.meta_description)
        .bind(&insight.canonical_url)
        .bind(&insight.og_image_url)
        .bind(published_at)
        .execute(pool)
        .await?;
    }

    println!("Default insights seeded successfully.");

    Ok(())
}

fn parse_published_at(value: &str) -> SeedResult<DateTime<Utc>> {
    let date = NaiveDate::parse_from_str(value, "%b %d, %Y")?;
    let datetime = date.and_hms_opt(0, 0, 0).ok_or("invalid published time")?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc))
}

fn estimated_view_count(index: usize, featured: bool) -> i32 {
    let base = if featured { 220 } else { 45 };
    let spread = ((index as i32 * 17) % 180) + (index as i32 / 2);
    base + spread
}
