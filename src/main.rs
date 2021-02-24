// use serde_json::value;
use std::env;

mod mailer;

#[tokio::main]
async fn main() {
    let _smtp_client = mailer::Mailer::new(
        env::var("SMTP_HOST").unwrap(),
        env::var("SMTP_AUTH_USER").unwrap(),
        env::var("SMTP_AUTH_PASSWD").unwrap(),
    );
}

// #[derive(Clone)]
// pub struct State {
//     mailer: mailer::Mailer,
// }

// async fn send_mail(mut req: tide::Request<State>) -> tide::Result<value::Value> {
//     let m: mailer::Message = req.body_json().await?;
//     // req.state().mailer.send_mail(&m).await?;
//     Ok(json!({
//         "code": 200,
//         "success": true
//     }))
// }
