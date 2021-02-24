use lettre::{
    transport::smtp::{authentication::Credentials, response::Response, Error},
    AsyncSmtpTransport, Message, Tokio1Connector, Tokio1Transport,
};
use std::result::Result;

pub struct Mailer {
    transport: AsyncSmtpTransport<Tokio1Connector>,
}

impl Mailer {
    pub fn new(smtp_server: String, smtp_auth_user: String, smtp_auth_passwd: String) -> Self {
        let creds = Credentials::new(smtp_auth_user, smtp_auth_passwd);

        let mailer = AsyncSmtpTransport::<Tokio1Connector>::relay(&smtp_server)
            .unwrap()
            .credentials(creds)
            .build();

        Self { transport: mailer }
    }

    pub async fn send(&self, message: Message) -> Result<Response, Error> {
        self.transport.send(message).await
    }
}
