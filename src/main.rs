use serde_json::value;
use std::env;
use tide::prelude::*;

mod mailer;

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::start();

    let smtp_client = mailer::Mailer::with_creds(
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
    mailer: mailer::Mailer,
}

async fn send_mail(mut req: tide::Request<State>) -> tide::Result<value::Value> {
    let m: mailer::Message = req.body_json().await?;
    req.state().mailer.send_mail(&m).await?;
    Ok(json!({
        "code": 200,
        "success": true
    }))
}
