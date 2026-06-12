use sqlx::PgPool;

type SeedResult<T> = Result<T, sqlx::Error>;

pub async fn seed_default_newsletter_subscribers(pool: &PgPool) -> SeedResult<()> {
    let existing_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM newsletter_subscribers
        "#,
    )
    .fetch_one(pool)
    .await?;

    if existing_count >= 233 {
        println!("Newsletter subscribers already seeded.");
        return Ok(());
    }

    sqlx::query(
        r#"
        INSERT INTO newsletter_subscribers (email, source, subscribed_at)
        SELECT
            format('subscriber%1$s@lkprofessionals-mail.local', gs.value),
            'seed',
            NOW() - make_interval(days => gs.value)
        FROM generate_series(1, 233) AS gs(value)
        ON CONFLICT (email) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    println!("Default newsletter subscribers seeded successfully.");

    Ok(())
}
