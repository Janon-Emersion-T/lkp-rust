use std::time::Duration;

use sqlx::{PgPool, Row};
use tokio::time::sleep;
use uuid::Uuid;

use super::newsletter::queue_insight_campaign;

const DEFAULT_POLL_SECONDS: u64 = 30;

pub async fn start_insight_scheduler_worker(db: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = publish_due_insights(&db).await {
                eprintln!("Insight scheduler error: {error}");
            }

            sleep(Duration::from_secs(DEFAULT_POLL_SECONDS)).await;
        }
    });
}

async fn publish_due_insights(pool: &PgPool) -> Result<(), sqlx::Error> {
    let due_insights = sqlx::query(
        r#"
        UPDATE insights
        SET published = TRUE,
            updated_at = NOW()
        WHERE published = FALSE
          AND published_at IS NOT NULL
          AND published_at <= NOW()
        RETURNING id, title, excerpt, slug
        "#,
    )
    .fetch_all(pool)
    .await?;

    for row in due_insights {
        let id: Uuid = row.get("id");
        let title: String = row.get("title");
        let excerpt: String = row.get("excerpt");
        let slug: String = row.get("slug");

        if let Err(error) =
            queue_insight_campaign(pool, id, &title, &excerpt, &format!("/insights/{slug}")).await
        {
            eprintln!("Failed to queue scheduled insight campaign: {error}");
        }
    }

    Ok(())
}
