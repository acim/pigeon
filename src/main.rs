#![allow(unused_imports)]
#![allow(dead_code)]

use anyhow;
use async_smtp::smtp::authentication;
use async_smtp::smtp::{error, response};
use async_smtp::{EmailAddress, Envelope, SendableEmail, SmtpClient};
use async_std::task;
use gethostname::gethostname;
use rand::Rng;
use serde::Deserialize;
use std::env;
use std::process;
use std::time::SystemTime;
use std::u32;
use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Message {
    from: String,
    to: String,
    subject: String,
    body: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let smtp_client = Mailer::with_creds(
        env::var("SMTP_AUTH_USER").unwrap(),
        env::var("SMTP_AUTH_PASSWD").unwrap(),
    );

    let mut app = tide::with_state(State {
        mailer: smtp_client,
    });
    app.at("/mail").post(send_mail);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[derive(Clone)]
pub struct State {
    mailer: Mailer,
}

#[derive(Clone)]
struct Mailer {
    creds: Option<authentication::Credentials>,
}

impl Mailer {
    fn new() -> Self {
        Self { creds: None }
    }

    fn with_creds(smtp_auth_user: String, smtp_auth_passwd: String) -> Self {
        Self {
            creds: Some(authentication::Credentials::new(
                smtp_auth_user,
                smtp_auth_passwd,
            )),
        }
    }

    async fn send_mail(&self, m: &Message) -> Result<response::Response, anyhow::Error> {
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

        let email =
            SendableEmail::new(envelope, "id".to_string(), message.to_string().into_bytes());

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

async fn send_mail(mut req: Request<State>) -> tide::Result {
    let m: Message = req.body_json().await?;
    req.state().mailer.send_mail(&m).await?;
    Ok(format!("{} {} {} {}", m.from, m.to, m.subject, m.body).into())
}
