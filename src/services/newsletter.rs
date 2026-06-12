use std::{env, time::Duration};

use chrono::Utc;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
    transport::smtp::authentication::Credentials,
};
use sqlx::{PgPool, Row};
use tokio::time::sleep;
use uuid::Uuid;

type ServiceResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const DEFAULT_BASE_URL: &str = "https://lkprofessionals.com";
const DEFAULT_FROM_NAME: &str = "LKProfessionals";
const DEFAULT_POLL_SECONDS: u64 = 20;
const DEFAULT_SEND_DELAY_MS: u64 = 1400;
const MAX_DELIVERY_ATTEMPTS: i32 = 5;

#[derive(Debug, Clone)]
pub struct NewsletterCampaignInput {
    pub title: String,
    pub subject: String,
    pub preview_text: Option<String>,
    pub content_html: String,
    pub cta_label: Option<String>,
    pub cta_url: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct PendingDelivery {
    delivery_id: Uuid,
    campaign_id: Uuid,
    email: String,
    attempts: i32,
    title: String,
    subject: String,
    preview_text: Option<String>,
    content_html: String,
    cta_label: Option<String>,
    cta_url: Option<String>,
}

#[derive(Debug, Clone)]
struct MailConfig {
    from_name: String,
    from_email: String,
    reply_to_email: Option<String>,
    smtp_host: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
}

pub async fn start_newsletter_worker(db: PgPool) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = process_pending_deliveries(&db).await {
                eprintln!("Newsletter worker error: {error}");
            }

            sleep(Duration::from_secs(DEFAULT_POLL_SECONDS)).await;
        }
    });
}

pub async fn queue_campaign(pool: &PgPool, input: NewsletterCampaignInput) -> ServiceResult<Uuid> {
    let mut tx = pool.begin().await?;

    let campaign_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO newsletter_campaigns
        (
            title,
            subject,
            preview_text,
            content_html,
            cta_label,
            cta_url,
            source_type,
            source_id,
            status,
            created_at,
            updated_at
        )
        VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8, 'queued', NOW(), NOW())
        ON CONFLICT (source_type, source_id)
        WHERE source_id IS NOT NULL
        DO UPDATE SET
            title = newsletter_campaigns.title
        RETURNING id
        "#,
    )
    .bind(&input.title)
    .bind(&input.subject)
    .bind(input.preview_text.as_deref())
    .bind(&input.content_html)
    .bind(input.cta_label.as_deref())
    .bind(input.cta_url.as_deref())
    .bind(&input.source_type)
    .bind(input.source_id)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO newsletter_deliveries
        (
            campaign_id,
            subscriber_id,
            email,
            status,
            created_at,
            updated_at
        )
        SELECT
            $1,
            subscribers.id,
            subscribers.email,
            'pending',
            NOW(),
            NOW()
        FROM newsletter_subscribers subscribers
        ON CONFLICT (campaign_id, email) DO NOTHING
        "#,
    )
    .bind(campaign_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(campaign_id)
}

pub async fn queue_insight_campaign(
    pool: &PgPool,
    source_id: Uuid,
    title: &str,
    excerpt: &str,
    public_url: &str,
) -> ServiceResult<Uuid> {
    queue_campaign(
        pool,
        NewsletterCampaignInput {
            title: format!("Insight: {title}"),
            subject: format!("New insight from LKProfessionals: {title}"),
            preview_text: Some(truncate_text(excerpt, 120)),
            content_html: render_content_email(
                "A new insight has been published on LKProfessionals.",
                excerpt,
            ),
            cta_label: Some("Read the Insight".to_string()),
            cta_url: Some(absolute_url(public_url)),
            source_type: "insight".to_string(),
            source_id: Some(source_id),
        },
    )
    .await
}

pub async fn queue_portfolio_campaign(
    pool: &PgPool,
    source_id: Uuid,
    title: &str,
    excerpt: &str,
    public_url: &str,
) -> ServiceResult<Uuid> {
    queue_campaign(
        pool,
        NewsletterCampaignInput {
            title: format!("Case study: {title}"),
            subject: format!("New case study from LKProfessionals: {title}"),
            preview_text: Some(truncate_text(excerpt, 120)),
            content_html: render_content_email(
                "A new case study has been published on LKProfessionals.",
                excerpt,
            ),
            cta_label: Some("View the Case Study".to_string()),
            cta_url: Some(absolute_url(public_url)),
            source_type: "portfolio".to_string(),
            source_id: Some(source_id),
        },
    )
    .await
}

pub fn plain_text_to_email_html(value: &str) -> String {
    text_to_html(value)
}

async fn process_pending_deliveries(pool: &PgPool) -> ServiceResult<()> {
    let Some(mail_config) = read_mail_config() else {
        return Ok(());
    };

    while let Some(delivery) = fetch_next_pending_delivery(pool).await? {
        if let Err(error) = mark_campaign_sending(pool, delivery.campaign_id).await {
            eprintln!("Failed to mark campaign sending: {error}");
        }

        match send_delivery(&mail_config, &delivery).await {
            Ok(_) => {
                mark_delivery_sent(pool, &delivery).await?;
                finalize_campaign_status(pool, delivery.campaign_id).await?;
            }
            Err(error) => {
                mark_delivery_failed(pool, &delivery, &error.to_string()).await?;
                finalize_campaign_status(pool, delivery.campaign_id).await?;
            }
        }

        sleep(Duration::from_millis(DEFAULT_SEND_DELAY_MS)).await;
    }

    Ok(())
}

async fn fetch_next_pending_delivery(pool: &PgPool) -> ServiceResult<Option<PendingDelivery>> {
    let mut tx = pool.begin().await?;

    let pending = sqlx::query_as::<_, PendingDelivery>(
        r#"
        SELECT
            deliveries.id AS delivery_id,
            deliveries.campaign_id,
            deliveries.email,
            deliveries.attempts,
            campaigns.title,
            campaigns.subject,
            campaigns.preview_text,
            campaigns.content_html,
            campaigns.cta_label,
            campaigns.cta_url
        FROM newsletter_deliveries deliveries
        INNER JOIN newsletter_campaigns campaigns
            ON campaigns.id = deliveries.campaign_id
        WHERE deliveries.status IN ('pending', 'retry')
          AND deliveries.attempts < $1
        ORDER BY deliveries.created_at ASC
        LIMIT 1
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .bind(MAX_DELIVERY_ATTEMPTS)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(ref delivery) = pending {
        sqlx::query(
            r#"
            UPDATE newsletter_deliveries
            SET status = 'sending',
                attempts = attempts + 1,
                last_attempt_at = NOW(),
                updated_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(delivery.delivery_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(pending)
}

async fn send_delivery(mail_config: &MailConfig, delivery: &PendingDelivery) -> ServiceResult<()> {
    let from_address = mail_config.from_email.parse()?;
    let to_address = delivery.email.parse()?;
    let from = Mailbox::new(Some(mail_config.from_name.clone()), from_address);
    let to = Mailbox::new(None, to_address);

    let mut builder = Message::builder()
        .from(from)
        .to(to)
        .subject(&delivery.subject);

    if let Some(reply_to) = mail_config.reply_to_email.as_deref() {
        builder = builder.reply_to(reply_to.parse()?);
    }

    let email = builder.multipart(
        MultiPart::alternative()
            .singlepart(SinglePart::plain(build_plain_text(delivery)))
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(render_email_layout(delivery)),
            ),
    )?;

    mailer(mail_config).send(email).await?;

    Ok(())
}

async fn mark_campaign_sending(pool: &PgPool, campaign_id: Uuid) -> ServiceResult<()> {
    sqlx::query(
        r#"
        UPDATE newsletter_campaigns
        SET status = 'sending',
            updated_at = NOW()
        WHERE id = $1
          AND status = 'queued'
        "#,
    )
    .bind(campaign_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn mark_delivery_sent(pool: &PgPool, delivery: &PendingDelivery) -> ServiceResult<()> {
    sqlx::query(
        r#"
        UPDATE newsletter_deliveries
        SET status = 'sent',
            sent_at = NOW(),
            last_error = NULL,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(delivery.delivery_id)
    .execute(pool)
    .await?;

    Ok(())
}

async fn mark_delivery_failed(
    pool: &PgPool,
    delivery: &PendingDelivery,
    error: &str,
) -> ServiceResult<()> {
    let status = if delivery.attempts + 1 >= MAX_DELIVERY_ATTEMPTS {
        "failed"
    } else {
        "retry"
    };

    sqlx::query(
        r#"
        UPDATE newsletter_deliveries
        SET status = $2,
            last_error = $3,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(delivery.delivery_id)
    .bind(status)
    .bind(truncate_text(error, 1000))
    .execute(pool)
    .await?;

    Ok(())
}

async fn finalize_campaign_status(pool: &PgPool, campaign_id: Uuid) -> ServiceResult<()> {
    let stats = sqlx::query(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE status IN ('pending', 'retry', 'sending')) AS open_count,
            COUNT(*) FILTER (WHERE status = 'failed') AS failed_count,
            COUNT(*) FILTER (WHERE status = 'sent') AS sent_count
        FROM newsletter_deliveries
        WHERE campaign_id = $1
        "#,
    )
    .bind(campaign_id)
    .fetch_one(pool)
    .await?;

    let open_count: i64 = stats.try_get("open_count")?;
    let failed_count: i64 = stats.try_get("failed_count")?;
    let sent_count: i64 = stats.try_get("sent_count")?;

    let (status, sent_at) = if open_count == 0 && sent_count > 0 && failed_count == 0 {
        ("sent", Some(Utc::now()))
    } else if open_count == 0 && failed_count > 0 {
        ("failed", None)
    } else {
        ("sending", None)
    };

    sqlx::query(
        r#"
        UPDATE newsletter_campaigns
        SET status = $2,
            sent_at = COALESCE($3, sent_at),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(campaign_id)
    .bind(status)
    .bind(sent_at)
    .execute(pool)
    .await?;

    Ok(())
}

fn mailer(mail_config: &MailConfig) -> AsyncSmtpTransport<Tokio1Executor> {
    let credentials = Credentials::new(
        mail_config.smtp_username.clone(),
        mail_config.smtp_password.clone(),
    );

    AsyncSmtpTransport::<Tokio1Executor>::relay(&mail_config.smtp_host)
        .unwrap()
        .credentials(credentials)
        .port(mail_config.smtp_port)
        .build()
}

fn read_mail_config() -> Option<MailConfig> {
    Some(MailConfig {
        from_name: env::var("MAIL_FROM_NAME").unwrap_or_else(|_| DEFAULT_FROM_NAME.to_string()),
        from_email: env::var("MAIL_FROM_ADDRESS").ok()?,
        reply_to_email: env::var("MAIL_REPLY_TO_ADDRESS").ok(),
        smtp_host: env::var("SMTP_HOST").ok()?,
        smtp_port: env::var("SMTP_PORT")
            .ok()
            .and_then(|value| value.parse().ok())
            .unwrap_or(587),
        smtp_username: env::var("SMTP_USERNAME").ok()?,
        smtp_password: env::var("SMTP_PASSWORD").ok()?,
    })
}

fn render_content_email(intro: &str, body: &str) -> String {
    format!(
        "<p style=\"margin:0 0 16px;color:#334155;font-size:16px;line-height:1.7;\">{}</p>{}",
        escape_html(intro),
        text_to_html(body),
    )
}

fn render_email_layout(delivery: &PendingDelivery) -> String {
    let cta = if let (Some(label), Some(url)) = (
        delivery
            .cta_label
            .as_deref()
            .filter(|value| !value.trim().is_empty()),
        delivery
            .cta_url
            .as_deref()
            .filter(|value| !value.trim().is_empty()),
    ) {
        format!(
            "<p style=\"margin:28px 0 0;\"><a href=\"{}\" style=\"display:inline-block;border-radius:999px;background:#22d3ee;padding:14px 24px;color:#082f49;font-weight:700;text-decoration:none;\">{}</a></p>",
            escape_html(url),
            escape_html(label)
        )
    } else {
        String::new()
    };

    let preview = delivery
        .preview_text
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        .map(|value| {
            format!(
                "<p style=\"margin:16px 0 0;color:#64748b;font-size:14px;line-height:1.7;\">{}</p>",
                escape_html(value)
            )
        })
        .unwrap_or_default();

    format!(
        "<!doctype html><html><body style=\"margin:0;background:#e2e8f0;font-family:Arial,sans-serif;\"><div style=\"max-width:680px;margin:0 auto;padding:32px 16px;\"><div style=\"border-radius:28px;background:#ffffff;padding:40px 32px;box-shadow:0 20px 60px rgba(15,23,42,0.12);\"><p style=\"margin:0;color:#0891b2;font-size:12px;font-weight:700;letter-spacing:0.24em;text-transform:uppercase;\">LKProfessionals</p><h1 style=\"margin:16px 0 0;color:#0f172a;font-size:32px;line-height:1.15;\">{}</h1>{}<div style=\"margin-top:24px;\">{}</div>{}<p style=\"margin:36px 0 0;color:#94a3b8;font-size:13px;line-height:1.6;\">You are receiving this email because you subscribed to updates from LKProfessionals.</p></div></div></body></html>",
        escape_html(&delivery.title),
        preview,
        delivery.content_html,
        cta
    )
}

fn build_plain_text(delivery: &PendingDelivery) -> String {
    let mut content = vec![delivery.title.clone()];

    if let Some(preview) = delivery.preview_text.as_deref() {
        if !preview.trim().is_empty() {
            content.push(preview.trim().to_string());
        }
    }

    let body = strip_html(&delivery.content_html);
    if !body.trim().is_empty() {
        content.push(body);
    }

    if let (Some(label), Some(url)) = (
        delivery
            .cta_label
            .as_deref()
            .filter(|value| !value.trim().is_empty()),
        delivery
            .cta_url
            .as_deref()
            .filter(|value| !value.trim().is_empty()),
    ) {
        content.push(format!("{label}: {url}"));
    }

    content.join("\n\n")
}

fn text_to_html(value: &str) -> String {
    value
        .split("\n\n")
        .filter_map(|paragraph| {
            let trimmed = paragraph.trim();
            (!trimmed.is_empty()).then(|| {
                format!(
                    "<p style=\"margin:0 0 16px;color:#334155;font-size:16px;line-height:1.7;\">{}</p>",
                    escape_html(trimmed).replace('\n', "<br>")
                )
            })
        })
        .collect::<Vec<_>>()
        .join("")
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn strip_html(value: &str) -> String {
    let mut stripped = String::with_capacity(value.len());
    let mut in_tag = false;

    for character in value.chars() {
        match character {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => stripped.push(character),
            _ => {}
        }
    }

    stripped
}

fn truncate_text(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }

    let truncated = trimmed.chars().take(max_chars).collect::<String>();
    format!("{}...", truncated.trim_end())
}

fn absolute_url(path_or_url: &str) -> String {
    if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
        path_or_url.to_string()
    } else {
        format!(
            "{}{}",
            env::var("APP_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string()),
            path_or_url
        )
    }
}
