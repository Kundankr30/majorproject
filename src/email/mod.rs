use lettre::{
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};
use std::env;

pub struct EmailService {
    mailer: SmtpTransport,
}

impl EmailService {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let smtp_server = env::var("SMTP_SERVER").unwrap_or_else(|_| "smtp.gmail.com".to_string());
        let smtp_port = env::var("SMTP_PORT").unwrap_or_else(|_| "587".to_string());
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        let creds = Credentials::new(smtp_username, smtp_password);

        let mailer = SmtpTransport::relay(&smtp_server)?
            .port(smtp_port.parse()?)
            .credentials(creds)
            .build();

        Ok(EmailService { mailer })
    }

    pub async fn send_ticket_created(&self, to_email: &str, ticket_id: &str, subject: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(env::var("FROM_EMAIL").expect("FROM_EMAIL must be set").parse()?)
            .to(to_email.parse()?)
            .subject("New Support Ticket Created")
            .body(format!(
                "Your support ticket has been created successfully.\n\nTicket ID: {}\nSubject: {}\n\nWe will get back to you soon.",
                ticket_id, subject
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub async fn send_ticket_updated(&self, to_email: &str, ticket_id: &str, status: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(env::var("FROM_EMAIL").expect("FROM_EMAIL must be set").parse()?)
            .to(to_email.parse()?)
            .subject("Support Ticket Updated")
            .body(format!(
                "Your support ticket has been updated.\n\nTicket ID: {}\nNew Status: {}\n\nPlease check your ticket for more details.",
                ticket_id, status
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }

    pub async fn send_comment_notification(&self, to_email: &str, ticket_id: &str, comment: &str) -> Result<(), Box<dyn std::error::Error>> {
        let email = Message::builder()
            .from(env::var("FROM_EMAIL").expect("FROM_EMAIL must be set").parse()?)
            .to(to_email.parse()?)
            .subject("New Comment on Your Ticket")
            .body(format!(
                "A new comment has been added to your support ticket.\n\nTicket ID: {}\nComment: {}\n\nPlease log in to view the full conversation.",
                ticket_id, comment
            ))?;

        self.mailer.send(&email)?;
        Ok(())
    }
} 