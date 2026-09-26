use axum::{
    extract::{Form, Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect},
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    models::{ContactMessage, LeadFilters, LeadStats},
    services::lead_notifications::send_new_lead_notification,
    state::AppState,
};

use super::{
    render::render,
    templates::{
        DashboardContactMessageShowTemplate, DashboardContactMessagesTemplate,
        DashboardLeadsTemplate,
    },
};

#[derive(Debug, Deserialize)]
pub struct ContactMessageForm {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub service_interest: Option<String>,
    pub budget_range: Option<String>,
    pub project_timeline: Option<String>,
    pub subject: String,
    pub message: String,
    pub source: Option<String>,
    pub redirect_to: Option<String>,

    // Honeypot field. Real users will never fill this.
    // Bots usually fill every input they see.
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LeadQuery {
    pub status: Option<String>,
    pub priority: Option<String>,
    pub service: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LeadUpdateForm {
    pub status: String,
    pub priority: String,
    pub lead_score: i32,
    pub assigned_to: Option<String>,
    pub internal_note: Option<String>,
    pub admin_reply: Option<String>,
    pub next_follow_up_at: Option<String>,
    pub lost_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct QuickStatusForm {
    pub status: String,
    pub redirect_to: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BlockSenderForm {
    pub redirect_to: Option<String>,
    pub reason: Option<String>,
}

fn clean_optional(value: &Option<String>) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn is_business_email(email: &str) -> bool {
    let email = email.trim().to_lowercase();

    let Some((_, domain)) = email.rsplit_once('@') else {
        return false;
    };

    let free_email_domains = [
        "gmail.com",
        "yahoo.com",
        "yahoo.co.uk",
        "hotmail.com",
        "outlook.com",
        "live.com",
        "icloud.com",
        "aol.com",
        "proton.me",
        "protonmail.com",
        "gmx.com",
        "mail.com",
    ];

    !free_email_domains.contains(&domain)
}

fn calculate_risk_penalty(form: &ContactMessageForm) -> i32 {
    let mut penalty = 0;

    let name = form.name.trim().to_lowercase();
    let subject = form.subject.trim().to_lowercase();
    let message = form.message.trim().to_lowercase();

    let combined = format!("{subject} {message}");

    // -------------------------------------------------
    // 1. COMMON UNSOLICITED SALES / SEO SPAM
    // -------------------------------------------------

    let spam_phrases = [
        "guest post",
        "guest posting",
        "link insertion",
        "backlinks",
        "backlink service",
        "seo package",
        "seo services",
        "increase your traffic",
        "increase website traffic",
        "first page of google",
        "rank on google",
        "domain authority",
        "domain rating",
        "casino links",
        "crypto investment",
        "investment opportunity",
    ];

    let spam_matches = spam_phrases
        .iter()
        .filter(|phrase| combined.contains(**phrase))
        .count();

    if spam_matches >= 2 {
        penalty += 25;
    } else if spam_matches == 1 {
        penalty += 10;
    }

    // -------------------------------------------------
    // 2. SUSPICIOUSLY LARGE NUMBER OF LINKS
    // -------------------------------------------------

    let link_count = combined.matches("http://").count()
        + combined.matches("https://").count()
        + combined.matches("www.").count();

    if link_count >= 3 {
        penalty += 20;
    } else if link_count >= 2 {
        penalty += 10;
    }

    // -------------------------------------------------
    // 3. OBVIOUS PLACEHOLDER / FAKE VALUES
    // -------------------------------------------------

    let fake_names = [
        "test", "testing", "asdf", "qwerty", "admin", "unknown", "none", "n/a",
    ];

    if fake_names.contains(&name.as_str()) {
        penalty += 20;
    }

    // -------------------------------------------------
    // 4. EXTREMELY LOW INFORMATION
    // -------------------------------------------------

    if message.len() < 20 {
        penalty += 5;
    }

    penalty
}

fn calculate_lead_score(form: &ContactMessageForm, source: &str) -> i32 {
    let mut score: i32 = 10;

    // -------------------------------------------------
    // 1. CONTACT QUALITY
    // -------------------------------------------------

    // Business-domain email is a useful trust signal.
    // Free email providers are NOT penalised.
    if is_business_email(&form.email) {
        score += 7;
    }

    // A phone / WhatsApp number gives us another
    // legitimate way to contact the prospect.
    if clean_optional(&form.phone).is_some() {
        score += 5;
    }

    // Company / brand information improves qualification.
    if clean_optional(&form.company).is_some() {
        score += 3;
    }

    // Selecting a service shows some understanding
    // of what the prospect needs.
    if clean_optional(&form.service_interest).is_some() {
        score += 5;
    }

    // -------------------------------------------------
    // 2. ENQUIRY SOURCE / COMMERCIAL INTENT
    // -------------------------------------------------

    let source = source.trim().to_lowercase();

    // Requesting a quote is stronger commercial intent
    // than submitting a general contact message.
    if source == "request_quote_modal" {
        score += 7;
    }

    // -------------------------------------------------
    // 3. PROJECT TIMELINE
    // -------------------------------------------------

    if let Some(timeline) = form.project_timeline.as_deref() {
        match timeline.trim().to_lowercase().as_str() {
            // Prospect says the project is urgent.
            "urgent" => {
                score += 12;
            }

            // Prospect expects to start this month.
            "this_month" => {
                score += 10;
            }

            // Contact-page option.
            "1_3_months" => {
                score += 6;
            }

            // Quote-modal option.
            "next_30_60_days" => {
                score += 7;
            }

            // Quote-modal option.
            "this_quarter" => {
                score += 5;
            }

            // Early-stage prospect.
            "planning" => {
                score += 2;
            }

            // Timeline exists but customer is flexible.
            "flexible" => {
                score += 2;
            }

            _ => {}
        }
    }

    // -------------------------------------------------
    // 4. MESSAGE / PROJECT DETAIL
    // -------------------------------------------------

    let message = form.message.trim();
    let message_lower = message.to_lowercase();

    // Reward useful project information.
    if message.len() >= 300 {
        score += 15;
    } else if message.len() >= 150 {
        score += 10;
    } else if message.len() >= 60 {
        score += 5;
    }

    // -------------------------------------------------
    // 5. COMMERCIAL / BUYING INTENT
    // -------------------------------------------------

    // Prospect is asking about money, quotation,
    // proposal, or commercial engagement.
    if message_lower.contains("quote")
        || message_lower.contains("quotation")
        || message_lower.contains("proposal")
        || message_lower.contains("estimate")
        || message_lower.contains("pricing")
        || message_lower.contains("price")
        || message_lower.contains("cost")
    {
        score += 8;
    }

    // Prospect explicitly wants direct communication.
    if message_lower.contains("call me")
        || message_lower.contains("contact me")
        || message_lower.contains("whatsapp")
        || message_lower.contains("meeting")
        || message_lower.contains("schedule a call")
        || message_lower.contains("book a call")
    {
        score += 5;
    }

    // Strong project-start language.
    if message_lower.contains("start project")
        || message_lower.contains("start the project")
        || message_lower.contains("get started")
        || message_lower.contains("ready to start")
        || message_lower.contains("need a website")
        || message_lower.contains("need website")
        || message_lower.contains("need a developer")
        || message_lower.contains("looking for a developer")
        || message_lower.contains("looking for an agency")
        || message_lower.contains("looking for a company")
    {
        score += 7;
    }

    // -------------------------------------------------
    // 6. WEAK / LOW-INFORMATION ENQUIRIES
    // -------------------------------------------------

    // Very short enquiries provide little qualification
    // information and should not become hot leads simply
    // because optional fields were completed.
    if message.len() < 30 {
        score -= 5;
    }

    // -------------------------------------------------
    // 7. RISK / SPAM SIGNALS
    // -------------------------------------------------

    score -= calculate_risk_penalty(form);

    // -------------------------------------------------
    // FINAL SCORE
    // -------------------------------------------------

    score.clamp(0, 100)
}

fn calculate_priority(form: &ContactMessageForm, score: i32) -> &'static str {
    let timeline = form
        .project_timeline
        .as_deref()
        .unwrap_or("")
        .to_lowercase();

    // Urgent enquiries require quick attention regardless
    // of whether they are fully qualified yet.
    if timeline.contains("urgent") {
        return "high";
    }

    // Strongly qualified leads also deserve high priority.
    if score >= 70 {
        return "high";
    }

    // Near-term projects or reasonably qualified enquiries.
    if timeline.contains("this_month") || score >= 40 {
        return "medium";
    }

    "normal"
}

fn parse_filters(query: LeadQuery) -> LeadFilters {
    LeadFilters {
        status: query.status.unwrap_or_else(|| "all".to_string()),
        priority: query.priority.unwrap_or_else(|| "all".to_string()),
        service: query.service.unwrap_or_else(|| "all".to_string()),
        q: query.q.unwrap_or_default(),
    }
}

fn parse_follow_up(value: Option<String>) -> Option<DateTime<Utc>> {
    let value = value?.trim().to_string();

    if value.is_empty() {
        return None;
    }

    DateTime::parse_from_rfc3339(&format!("{value}:00Z"))
        .ok()
        .map(|value| value.with_timezone(&Utc))
}

fn current_status_timestamp_sql(status: &str) -> &'static str {
    match status {
        "contacted" => "contacted_at = COALESCE(contacted_at, NOW()),",
        "qualified" => "qualified_at = COALESCE(qualified_at, NOW()),",
        "converted" => "converted_at = COALESCE(converted_at, NOW()),",
        "archived" => "archived_at = COALESCE(archived_at, NOW()),",
        "spam" => "spam_at = COALESCE(spam_at, NOW()),",
        _ => "",
    }
}

fn client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .or_else(|| headers.get("x-real-ip"))
        .and_then(|value| value.to_str().ok())
        .map(|value| value.split(',').next().unwrap_or(value).trim().to_string())
}

fn user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

fn safe_public_redirect(value: Option<&str>, fallback: &str) -> String {
    value
        .map(str::trim)
        .filter(|value| value.starts_with('/'))
        .filter(|value| !value.starts_with("//"))
        .filter(|value| !value.starts_with("/dashboard"))
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| fallback.to_string())
}

fn clean_source(value: &Option<String>, fallback: &str) -> String {
    clean_optional(value).unwrap_or_else(|| fallback.to_string())
}

async fn insert_contact_message(
    state: &AppState,
    headers: &HeaderMap,
    form: &ContactMessageForm,
    fallback_source: &str,
) -> Result<ContactMessage, sqlx::Error> {
    let name = form.name.trim();
    let email = form.email.trim().to_lowercase();
    let subject = form.subject.trim();
    let message = form.message.trim();

    // Resolve the source once and use the same value for
    // scoring and database storage.
    let source = clean_source(&form.source, fallback_source);

    // Capture visitor IP once so it can be used for both
    // abuse detection and database storage.
    let visitor_ip = client_ip(headers);

    // -------------------------------------------------
    // INITIAL LEAD SCORE
    // -------------------------------------------------

    let mut lead_score = calculate_lead_score(form, &source);

    // -------------------------------------------------
    // REPEATED EMAIL DETECTION
    // -------------------------------------------------

    // Multiple submissions are not automatically spam.
    // A genuine prospect may retry the form or follow up.
    let recent_email_submissions = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM contact_messages
        WHERE lower(email) = lower($1)
          AND created_at >= NOW() - INTERVAL '24 hours'
        "#,
    )
    .bind(&email)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    if recent_email_submissions >= 3 {
        lead_score -= 20;
    } else if recent_email_submissions >= 1 {
        lead_score -= 5;
    }

    // -------------------------------------------------
    // IDENTICAL MESSAGE DETECTION
    // -------------------------------------------------

    // Repeated identical messages from the same email
    // are a stronger spam / automation signal.
    let identical_message_count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM contact_messages
        WHERE lower(email) = lower($1)
          AND lower(btrim(message)) = lower(btrim($2))
          AND created_at >= NOW() - INTERVAL '7 days'
        "#,
    )
    .bind(&email)
    .bind(message)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    if identical_message_count >= 2 {
        lead_score -= 25;
    } else if identical_message_count >= 1 {
        lead_score -= 10;
    }

    // -------------------------------------------------
    // IP-BASED ABUSE DETECTION
    // -------------------------------------------------

    if let Some(ref ip) = visitor_ip {
        let recent_ip_submissions = sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM contact_messages
            WHERE client_ip = $1
              AND created_at >= NOW() - INTERVAL '1 hour'
            "#,
        )
        .bind(ip)
        .fetch_one(&state.db)
        .await
        .unwrap_or(0);

        // Be conservative because offices, mobile networks,
        // schools and other organisations may share an IP.
        if recent_ip_submissions >= 10 {
            lead_score -= 35;
        } else if recent_ip_submissions >= 5 {
            lead_score -= 20;
        } else if recent_ip_submissions >= 3 {
            lead_score -= 10;
        }
    }

    // -------------------------------------------------
    // FINALISE SCORE
    // -------------------------------------------------

    // Ensure PostgreSQL always receives a valid 0-100 score.
    lead_score = lead_score.clamp(0, 100);

    // Priority is separate from lead quality.
    // An urgent enquiry can therefore receive high operational
    // priority even when it is not yet strongly qualified.
    let priority = calculate_priority(form, lead_score);

    // -------------------------------------------------
    // BLOCKED SENDER CHECK
    // -------------------------------------------------

    let sender_blocked = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM blocked_contact_senders
            WHERE lower(email) = lower($1)
        )
        "#,
    )
    .bind(&email)
    .fetch_one(&state.db)
    .await
    .unwrap_or(false);

    let status = if sender_blocked { "spam" } else { "new" };

    let lost_reason = if sender_blocked {
        Some("Automatically blocked sender".to_string())
    } else {
        None
    };

    // -------------------------------------------------
    // STORE LEAD
    // -------------------------------------------------

    let lead = sqlx::query_as::<_, ContactMessage>(
        r#"
        INSERT INTO contact_messages
        (
            name,
            email,
            phone,
            company,
            service_interest,
            budget_range,
            project_timeline,
            subject,
            message,
            source,
            status,
            priority,
            lead_score,
            lost_reason,
            spam_at,
            client_ip,
            user_agent
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14,
            CASE WHEN $11 = 'spam' THEN NOW() ELSE NULL END,
            $15, $16
        )
        RETURNING *
        "#,
    )
    .bind(name)
    .bind(&email)
    .bind(clean_optional(&form.phone))
    .bind(clean_optional(&form.company))
    .bind(clean_optional(&form.service_interest))
    .bind(clean_optional(&form.budget_range))
    .bind(clean_optional(&form.project_timeline))
    .bind(subject)
    .bind(message)
    .bind(source)
    .bind(status)
    .bind(priority)
    .bind(lead_score)
    .bind(lost_reason)
    .bind(visitor_ip)
    .bind(user_agent(headers))
    .fetch_one(&state.db)
    .await?;

    Ok(lead)
}

pub async fn submit_contact_message(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<ContactMessageForm>,
) -> impl IntoResponse {
    let name = form.name.trim();
    let email = form.email.trim().to_lowercase();
    let subject = form.subject.trim();
    let message = form.message.trim();

    // Silent bot trap.
    // If this hidden field has a value, act successful but do not store the lead.
    if clean_optional(&form.website).is_some() {
        return Redirect::to("/contact?success=1").into_response();
    }

    // Basic abuse protection.
    // Keep this simple for now. Later we can add rate limiting.
    if name.len() > 120 || email.len() > 180 || subject.len() > 180 || message.len() > 5000 {
        return (
            StatusCode::BAD_REQUEST,
            "Your message is too long. Please shorten it and try again.",
        )
            .into_response();
    }

    if name.len() < 2 || email.len() < 5 || subject.len() < 3 || message.len() < 10 {
        return (
            StatusCode::BAD_REQUEST,
            "Please complete the contact form correctly.",
        )
            .into_response();
    }

    match insert_contact_message(&state, &headers, &form, "contact_page").await {
        Ok(lead) => {
            if let Err(error) = send_new_lead_notification(&lead).await {
                eprintln!(
                    "Failed to send lead notification for contact message {}: {error}",
                    lead.id
                );
            }

            Redirect::to("/contact?success=1").into_response()
        }
        Err(error) => {
            eprintln!("Failed to submit contact message: {error}");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to submit contact message.",
            )
                .into_response()
        }
    }
}

pub async fn submit_request_quote(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<ContactMessageForm>,
) -> impl IntoResponse {
    let name = form.name.trim();
    let email = form.email.trim().to_lowercase();
    let message = form.message.trim();
    let subject = if form.subject.trim().is_empty() {
        "Project Quote Request"
    } else {
        form.subject.trim()
    };

    if clean_optional(&form.website).is_some() {
        let redirect_to =
            safe_public_redirect(form.redirect_to.as_deref(), "/?request_quote=success");
        return Redirect::to(&redirect_to).into_response();
    }

    if name.len() > 120 || email.len() > 180 || subject.len() > 180 || message.len() > 5000 {
        return (
            StatusCode::BAD_REQUEST,
            "Your request is too long. Please shorten it and try again.",
        )
            .into_response();
    }

    if name.len() < 2 || email.len() < 5 || message.len() < 10 {
        return (
            StatusCode::BAD_REQUEST,
            "Please complete the quote request correctly.",
        )
            .into_response();
    }

    match insert_contact_message(&state, &headers, &form, "request_quote_modal").await {
        Ok(lead) => {
            if let Err(error) = send_new_lead_notification(&lead).await {
                eprintln!(
                    "Failed to send lead notification for quote request {}: {error}",
                    lead.id
                );
            }

            let redirect_to =
                safe_public_redirect(form.redirect_to.as_deref(), "/?request_quote=success");
            Redirect::to(&redirect_to).into_response()
        }
        Err(error) => {
            eprintln!("Failed to submit quote request: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to submit quote request.",
            )
                .into_response()
        }
    }
}

async fn fetch_stats(state: &AppState) -> LeadStats {
    sqlx::query_as::<_, LeadStats>(
        r#"
        SELECT
            COUNT(*) AS total,
            COUNT(*) FILTER (WHERE status = 'new') AS new_count,
            COUNT(*) FILTER (WHERE status = 'contacted') AS contacted_count,
            COUNT(*) FILTER (WHERE status = 'qualified') AS qualified_count,
            COUNT(*) FILTER (WHERE status = 'converted') AS converted_count,
            COUNT(*) FILTER (WHERE status = 'archived') AS archived_count,
            COUNT(*) FILTER (WHERE status = 'spam') AS spam_count,
            COUNT(*) FILTER (WHERE priority = 'high') AS high_priority_count,
            AVG(lead_score)::float8 AS avg_score
        FROM contact_messages
        "#,
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or(LeadStats {
        total: 0,
        new_count: 0,
        contacted_count: 0,
        qualified_count: 0,
        converted_count: 0,
        archived_count: 0,
        spam_count: 0,
        high_priority_count: 0,
        avg_score: Some(0.0),
    })
}

async fn fetch_filtered_messages(state: &AppState, filters: &LeadFilters) -> Vec<ContactMessage> {
    let q = format!("%{}%", filters.q.trim());

    sqlx::query_as::<_, ContactMessage>(
        r#"
        SELECT *
        FROM contact_messages
        WHERE
            ($1 = 'all' OR status = $1)
            AND ($2 = 'all' OR priority = $2)
            AND ($3 = 'all' OR service_interest = $3)
            AND (
                $4 = '%%'
                OR name ILIKE $4
                OR email ILIKE $4
                OR phone ILIKE $4
                OR company ILIKE $4
                OR subject ILIKE $4
                OR message ILIKE $4
            )
        ORDER BY
            CASE priority
                WHEN 'high' THEN 1
                WHEN 'medium' THEN 2
                ELSE 3
            END,
            lead_score DESC,
            created_at DESC
        LIMIT 150
        "#,
    )
    .bind(&filters.status)
    .bind(&filters.priority)
    .bind(&filters.service)
    .bind(q)
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|error| {
        eprintln!("Failed to fetch filtered contact messages: {error}");
        Vec::new()
    })
}

pub async fn dashboard_leads(
    State(state): State<AppState>,
    Query(query): Query<LeadQuery>,
) -> impl IntoResponse {
    let filters = parse_filters(query);
    let stats = fetch_stats(&state).await;
    let leads = fetch_filtered_messages(&state, &filters).await;

    render(DashboardLeadsTemplate {
        leads,
        stats,
        filters,
    })
}

pub async fn dashboard_contact_messages(
    State(state): State<AppState>,
    Query(query): Query<LeadQuery>,
) -> impl IntoResponse {
    let filters = parse_filters(query);
    let stats = fetch_stats(&state).await;
    let messages = fetch_filtered_messages(&state, &filters).await;

    render(DashboardContactMessagesTemplate {
        messages,
        stats,
        filters,
    })
}

pub async fn dashboard_contact_message_show(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let message = sqlx::query_as::<_, ContactMessage>(
        r#"
        SELECT *
        FROM contact_messages
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await;

    match message {
        Ok(Some(message)) => {
            render(DashboardContactMessageShowTemplate { message }).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Contact message not found").into_response(),
        Err(error) => {
            eprintln!("Failed to fetch contact message: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch contact message.",
            )
                .into_response()
        }
    }
}

pub async fn dashboard_contact_message_update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<LeadUpdateForm>,
) -> impl IntoResponse {
    let status_marker = current_status_timestamp_sql(&form.status);
    let next_follow_up_at = parse_follow_up(form.next_follow_up_at);

    let query = format!(
        r#"
        UPDATE contact_messages
        SET
            status = $1,
            priority = $2,
            lead_score = $3,
            assigned_to = $4,
            internal_note = $5,
            admin_reply = $6,
            next_follow_up_at = $7,
            lost_reason = $8,
            {status_marker}
            updated_at = NOW()
        WHERE id = $9
        "#
    );

    let result = sqlx::query(&query)
        .bind(form.status)
        .bind(form.priority)
        .bind(form.lead_score.clamp(0, 100))
        .bind(clean_optional(&form.assigned_to))
        .bind(clean_optional(&form.internal_note))
        .bind(clean_optional(&form.admin_reply))
        .bind(next_follow_up_at)
        .bind(clean_optional(&form.lost_reason))
        .bind(id)
        .execute(&state.db)
        .await;

    if let Err(error) = result {
        eprintln!("Failed to update contact message: {error}");
    }

    Redirect::to(&format!("/dashboard/contact-messages/{id}"))
}

pub async fn dashboard_contact_message_quick_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<QuickStatusForm>,
) -> impl IntoResponse {
    let status_marker = current_status_timestamp_sql(&form.status);

    let query = format!(
        r#"
        UPDATE contact_messages
        SET
            status = $1,
            {status_marker}
            updated_at = NOW()
        WHERE id = $2
        "#
    );

    if let Err(error) = sqlx::query(&query)
        .bind(form.status)
        .bind(id)
        .execute(&state.db)
        .await
    {
        eprintln!("Failed to quick-update contact message: {error}");
    }

    let redirect_to = form
        .redirect_to
        .as_deref()
        .filter(|value| value.starts_with("/dashboard"))
        .unwrap_or("/dashboard/contact-messages");

    Redirect::to(redirect_to)
}

#[derive(Debug, Deserialize)]
pub struct ReplyForm {
    pub admin_reply: Option<String>,
}

pub async fn dashboard_contact_message_reply(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<ReplyForm>,
) -> impl IntoResponse {
    let result = sqlx::query(
        r#"
        UPDATE contact_messages
        SET
            admin_reply = $1,
            replied_at = NOW(),
            status = CASE
                WHEN status = 'new' THEN 'contacted'
                ELSE status
            END,
            contacted_at = CASE
                WHEN contacted_at IS NULL THEN NOW()
                ELSE contacted_at
            END,
            updated_at = NOW()
        WHERE id = $2
        "#,
    )
    .bind(clean_optional(&form.admin_reply))
    .bind(id)
    .execute(&state.db)
    .await;

    if let Err(error) = result {
        eprintln!("Failed to save contact message reply: {error}");
    }

    Redirect::to(&format!("/dashboard/contact-messages/{id}"))
}

pub async fn dashboard_contact_message_delete(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    if let Err(error) = sqlx::query(
        r#"
        DELETE FROM contact_messages
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&state.db)
    .await
    {
        eprintln!("Failed to delete contact message: {error}");
    }

    Redirect::to("/dashboard/contact-messages")
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteMessagesForm {
    pub ids: Vec<Uuid>,
}

pub async fn dashboard_contact_message_bulk_delete(
    State(state): State<AppState>,
    Form(form): Form<BulkDeleteMessagesForm>,
) -> impl IntoResponse {
    if !form.ids.is_empty() {
        if let Err(error) = sqlx::query("DELETE FROM contact_messages WHERE id = ANY($1)")
            .bind(&form.ids)
            .execute(&state.db)
            .await
        {
            eprintln!("Failed to bulk-delete contact messages: {error}");
        }
    }

    Redirect::to("/dashboard/contact-messages")
}

pub async fn dashboard_contact_message_block_sender(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Form(form): Form<BlockSenderForm>,
) -> impl IntoResponse {
    let message = sqlx::query_scalar::<_, String>(
        r#"
        SELECT email
        FROM contact_messages
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await;

    let Some(email) = (match message {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Failed to fetch sender email for block action: {error}");
            None
        }
    }) else {
        return Redirect::to("/dashboard/contact-messages");
    };

    let email = email.trim().to_lowercase();
    let reason =
        clean_optional(&form.reason).unwrap_or_else(|| "Blocked from spam/scam review".to_string());

    if let Err(error) = sqlx::query(
        r#"
        INSERT INTO blocked_contact_senders (email, reason, blocked_from_message_id, created_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        ON CONFLICT (email)
        DO UPDATE SET
            reason = EXCLUDED.reason,
            blocked_from_message_id = EXCLUDED.blocked_from_message_id,
            updated_at = NOW()
        "#,
    )
    .bind(&email)
    .bind(&reason)
    .bind(id)
    .execute(&state.db)
    .await
    {
        eprintln!("Failed to block sender email: {error}");
    }

    if let Err(error) = sqlx::query(
        r#"
        UPDATE contact_messages
        SET
            status = 'spam',
            spam_at = COALESCE(spam_at, NOW()),
            lost_reason = CASE
                WHEN lost_reason IS NULL OR btrim(lost_reason) = '' THEN $1
                ELSE lost_reason
            END,
            updated_at = NOW()
        WHERE lower(email) = $2
        "#,
    )
    .bind(&reason)
    .bind(&email)
    .execute(&state.db)
    .await
    {
        eprintln!("Failed to mark blocked sender messages as spam: {error}");
    }

    let redirect_to = form
        .redirect_to
        .as_deref()
        .filter(|value| value.starts_with("/dashboard"))
        .unwrap_or("/dashboard/contact-messages");

    Redirect::to(redirect_to)
}
