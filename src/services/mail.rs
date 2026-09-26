use std::env;

use lettre::{AsyncSmtpTransport, Tokio1Executor, transport::smtp::authentication::Credentials};

pub type ServiceResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const DEFAULT_BASE_URL: &str = "https://lkprofessionals.com";
const DEFAULT_FROM_NAME: &str = "LKProfessionals";

#[derive(Debug, Clone)]
pub struct MailConfig {
    pub from_name: String,
    pub from_email: String,
    pub reply_to_email: Option<String>,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

pub fn read_mail_config() -> Option<MailConfig> {
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

pub fn mailer(mail_config: &MailConfig) -> ServiceResult<AsyncSmtpTransport<Tokio1Executor>> {
    let credentials = Credentials::new(
        mail_config.smtp_username.clone(),
        mail_config.smtp_password.clone(),
    );

    let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&mail_config.smtp_host)?
        .credentials(credentials)
        .port(mail_config.smtp_port)
        .build();

    Ok(transport)
}

pub fn absolute_url(path_or_url: &str) -> String {
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

pub fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

pub fn strip_html(value: &str) -> String {
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

pub fn text_to_html(value: &str) -> String {
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

pub fn truncate_text(value: &str, max_chars: usize) -> String {
    let trimmed = value.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }

    let truncated = trimmed.chars().take(max_chars).collect::<String>();
    format!("{}...", truncated.trim_end())
}

pub fn sanitize_header(value: &str) -> String {
    value
        .chars()
        .filter(|character| !matches!(character, '\r' | '\n'))
        .collect::<String>()
        .trim()
        .to_string()
}
