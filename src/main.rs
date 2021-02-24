// use serde_json::value;
use std::env;
use lettre::Message;

mod mailer;

#[tokio::main]
async fn main() {
    let mailer = mailer::Mailer::new(
        env::var("SMTP_HOST").unwrap(),
        env::var("SMTP_AUTH_USER").unwrap(),
        env::var("SMTP_AUTH_PASSWD").unwrap(),
    );

    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse().unwrap())
        .reply_to("John Doe <jdoe@domain.tld>".parse().unwrap())
        .to("Hei <hei@domain.tld>".parse().unwrap())
        .subject("Happy new async year")
        .body(String::from("Be happy with async!"))
        .unwrap();

    mailer.send(email).await.unwrap();
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
