use lettre::{
    AsyncTransport, Message,
    message::{Mailbox, MultiPart, SinglePart, header::ContentType},
};

use crate::models::ContactMessage;

use super::mail::{
    ServiceResult, absolute_url, escape_html, mailer, read_mail_config, sanitize_header,
};

const INTERNAL_LEAD_RECIPIENT: &str = "info@lkprofessionals.com";
const LEAD_FROM_NAME: &str = "LKProfessionals Leads";

pub async fn send_new_lead_notification(lead: &ContactMessage) -> ServiceResult<()> {
    if lead.status == "spam" {
        return Ok(());
    }

    let Some(mail_config) = read_mail_config() else {
        return Ok(());
    };

    let from = Mailbox::new(
        Some(LEAD_FROM_NAME.to_string()),
        mail_config.from_email.parse()?,
    );
    let to = Mailbox::new(None, INTERNAL_LEAD_RECIPIENT.parse()?);
    let reply_to = Mailbox::new(Some(sanitize_header(&lead.name)), lead.email.parse()?);

    let email = Message::builder()
        .from(from)
        .to(to)
        .reply_to(reply_to)
        .subject(build_subject(lead))
        .multipart(
            MultiPart::alternative()
                .singlepart(SinglePart::plain(build_plain_text(lead)))
                .singlepart(
                    SinglePart::builder()
                        .header(ContentType::TEXT_HTML)
                        .body(render_html(lead)),
                ),
        )?;

    mailer(&mail_config)?.send(email).await?;

    Ok(())
}

fn build_subject(lead: &ContactMessage) -> String {
    let priority = lead.priority.to_uppercase();
    let target = lead
        .company
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| lead.name.trim());

    sanitize_header(&format!(
        "[{}] New {} Lead - {}",
        priority,
        lead.service_label(),
        target
    ))
}

fn build_plain_text(lead: &ContactMessage) -> String {
    let mut lines = vec![
        "NEW LKP LEAD".to_string(),
        String::new(),
        format!("Lead Score: {}/100", lead.lead_score),
        format!("Priority: {}", lead.priority.to_uppercase()),
        format!("Source: {}", lead.source_label()),
        format!("Created: {}", lead.created_at.format("%Y-%m-%d %H:%M UTC")),
        String::new(),
        "CONTACT".to_string(),
        format!("Name: {}", lead.name),
        format!("Company: {}", optional_text(lead.company.as_deref())),
        format!("Email: {}", lead.email),
        format!("Phone / WhatsApp: {}", optional_text(lead.phone.as_deref())),
        String::new(),
        "PROJECT".to_string(),
        format!("Service: {}", lead.service_label()),
        format!("Timeline: {}", lead.timeline_label()),
        format!("Subject: {}", lead.subject),
        String::new(),
        "MESSAGE".to_string(),
        lead.message.clone(),
        String::new(),
        "QUICK ACTIONS".to_string(),
        format!("Reply to Lead: mailto:{}", lead.email),
    ];

    if let Some(whatsapp_url) = lead.whatsapp_url() {
        lines.push(format!("WhatsApp: {whatsapp_url}"));
    }

    lines.push(format!("Open Lead in CRM: {}", crm_url(lead)));
    lines.push(format!("Lead source: {}", lead.source_label()));

    lines.join("\n")
}

fn render_html(lead: &ContactMessage) -> String {
    let priority_color = match lead.priority.as_str() {
        "high" => "#dc2626",
        "medium" => "#d97706",
        _ => "#475569",
    };
    let priority_background = match lead.priority.as_str() {
        "high" => "#fef2f2",
        "medium" => "#fffbeb",
        _ => "#f8fafc",
    };
    let priority_border = match lead.priority.as_str() {
        "high" => "#fecaca",
        "medium" => "#fde68a",
        _ => "#e2e8f0",
    };

    let company = optional_text(lead.company.as_deref());
    let phone = optional_text(lead.phone.as_deref());
    let crm_url = crm_url(lead);
    let mailto_url = format!("mailto:{}", escape_attr(&lead.email));
    let whatsapp_button = lead
        .whatsapp_url()
        .map(|url| action_button(&url, "WhatsApp", "#0f766e", "#ffffff"))
        .unwrap_or_default();

    format!(
        r#"<!doctype html>
<html>
<body style="margin:0;background:#e2e8f0;font-family:Arial,Helvetica,sans-serif;color:#0f172a;">
  <div style="max-width:720px;margin:0 auto;padding:24px 12px;">
    <div style="background:#0f172a;border-radius:24px 24px 0 0;padding:28px 24px;">
      <p style="margin:0;color:#22d3ee;font-size:12px;font-weight:700;letter-spacing:0.22em;text-transform:uppercase;">LKProfessionals</p>
      <h1 style="margin:12px 0 0;color:#ffffff;font-size:26px;line-height:1.2;">NEW LKP LEAD</h1>
      <p style="margin:10px 0 0;color:#cbd5e1;font-size:14px;line-height:1.6;">{source} from {created}</p>
    </div>
    <div style="background:#ffffff;border-radius:0 0 24px 24px;padding:24px;border:1px solid #cbd5e1;border-top:0;">
      <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="border-collapse:collapse;margin:0 0 20px;">
        <tr>
          <td style="width:50%;padding:14px;border:1px solid #e2e8f0;border-radius:16px;background:#f8fafc;">
            <div style="color:#64748b;font-size:12px;font-weight:700;text-transform:uppercase;">Lead Score</div>
            <div style="margin-top:6px;color:#0f172a;font-size:28px;font-weight:800;">{score}/100</div>
          </td>
          <td style="width:16px;"></td>
          <td style="width:50%;padding:14px;border:1px solid {priority_border};border-radius:16px;background:{priority_background};">
            <div style="color:#64748b;font-size:12px;font-weight:700;text-transform:uppercase;">Priority</div>
            <div style="margin-top:6px;color:{priority_color};font-size:28px;font-weight:800;">{priority}</div>
          </td>
        </tr>
      </table>

      {section_contact}
      {section_project}

      <div style="margin-top:18px;padding:18px;border:1px solid #e2e8f0;border-radius:16px;background:#ffffff;">
        <h2 style="margin:0 0 12px;color:#0f172a;font-size:15px;letter-spacing:0.12em;text-transform:uppercase;">Message</h2>
        <div style="color:#334155;font-size:15px;line-height:1.7;">{message}</div>
      </div>

      <div style="margin-top:18px;padding:18px;border:1px solid #cffafe;border-radius:16px;background:#ecfeff;">
        <h2 style="margin:0 0 14px;color:#0f172a;font-size:15px;letter-spacing:0.12em;text-transform:uppercase;">Quick Actions</h2>
        {reply_button}
        {whatsapp_button}
        {crm_button}
        <p style="margin:14px 0 0;color:#475569;font-size:13px;line-height:1.6;">Normal email reply is configured to go directly to {email}.</p>
      </div>
    </div>
  </div>
</body>
</html>"#,
        source = escape_html(lead.source_label()),
        created = escape_html(&lead.created_at.format("%Y-%m-%d %H:%M UTC").to_string()),
        score = lead.lead_score,
        priority = escape_html(&lead.priority.to_uppercase()),
        priority_color = priority_color,
        priority_background = priority_background,
        priority_border = priority_border,
        section_contact = info_section(
            "Contact",
            &[
                ("Name", &lead.name),
                ("Company", company),
                ("Email", &lead.email),
                ("Phone / WhatsApp", phone),
            ],
        ),
        section_project = info_section(
            "Project",
            &[
                ("Service", lead.service_label()),
                ("Timeline", lead.timeline_label()),
                ("Subject", &lead.subject),
                ("Source", lead.source_label()),
            ],
        ),
        message = text_block(&lead.message),
        reply_button = action_button(&mailto_url, "Reply to Lead", "#0891b2", "#ffffff"),
        whatsapp_button = whatsapp_button,
        crm_button = action_button(&crm_url, "Open Lead in CRM", "#0f172a", "#ffffff"),
        email = escape_html(&lead.email),
    )
}

fn info_section(title: &str, rows: &[(&str, &str)]) -> String {
    let rows = rows
        .iter()
        .map(|(label, value)| {
            format!(
                r#"<tr>
  <td style="padding:8px 0;color:#64748b;font-size:13px;font-weight:700;width:38%;vertical-align:top;">{}</td>
  <td style="padding:8px 0;color:#0f172a;font-size:14px;line-height:1.5;vertical-align:top;">{}</td>
</tr>"#,
                escape_html(label),
                escape_html(value)
            )
        })
        .collect::<Vec<_>>()
        .join("");

    format!(
        r#"<div style="margin-top:18px;padding:18px;border:1px solid #e2e8f0;border-radius:16px;background:#ffffff;">
  <h2 style="margin:0 0 10px;color:#0f172a;font-size:15px;letter-spacing:0.12em;text-transform:uppercase;">{}</h2>
  <table role="presentation" width="100%" cellspacing="0" cellpadding="0" style="border-collapse:collapse;">{}</table>
</div>"#,
        escape_html(title),
        rows
    )
}

fn action_button(url: &str, label: &str, background: &str, color: &str) -> String {
    format!(
        r#"<a href="{}" style="display:inline-block;margin:0 8px 8px 0;border-radius:999px;background:{};padding:12px 18px;color:{};font-size:14px;font-weight:800;text-decoration:none;">{}</a>"#,
        escape_attr(url),
        background,
        color,
        escape_html(label)
    )
}

fn text_block(value: &str) -> String {
    escape_html(value.trim()).replace('\n', "<br>")
}

fn optional_text(value: Option<&str>) -> &str {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("Not provided")
}

fn crm_url(lead: &ContactMessage) -> String {
    absolute_url(&format!("/dashboard/contact-messages/{}", lead.id))
}

fn escape_attr(value: &str) -> String {
    escape_html(value).replace(' ', "%20")
}
