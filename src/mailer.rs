use async_smtp::smtp::{authentication, response};
use async_smtp::{EmailAddress, Envelope, SendableEmail, SmtpClient};
use gethostname::gethostname;
use rand::Rng;
use serde::Deserialize;
use std::process;
use std::time::SystemTime;
use std::u32;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<String>,
    pub subject: String,
    pub body: String,
}

#[derive(Clone)]
pub struct Mailer {
    creds: Option<authentication::Credentials>,
}

impl Mailer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { creds: None }
    }

    pub fn with_creds(smtp_auth_user: String, smtp_auth_passwd: String) -> Self {
        Self {
            creds: Some(authentication::Credentials::new(
                smtp_auth_user,
                smtp_auth_passwd,
            )),
        }
    }

    pub async fn send_mail(&self, m: &Message) -> Result<response::Response, anyhow::Error> {
        let message = format!(
            "From: {}\nTo: {}\nMessage-ID: {}\nSubject: {}\n\n{}",
            &m.from,
            &m.to,
            self.message_id()?,
            m.subject,
            m.body
        );

        let envelope = Envelope::new(
            Some(EmailAddress::new(m.from.clone())?),
            vec![EmailAddress::new(m.to.clone())?],
        )?;

        let email = SendableEmail::new(envelope, "id".to_string(), message.into_bytes());

        let mut smtp = SmtpClient::new("smtp.ectobit.com").await?;

        if let Some(creds) = &self.creds {
            smtp = smtp.credentials(creds.clone());
        }

        let result = smtp.into_transport().connect_and_send(email).await?;

        Ok(result)
    }

    fn message_id(&self) -> anyhow::Result<String> {
        let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

        let mut rng = rand::thread_rng();
        let r: u32 = rng.gen();

        Ok(format!(
            "<{}.{}.{}@{}>",
            t.as_nanos(),
            process::id(),
            r,
            gethostname().into_string().unwrap()
        ))
    }
}
