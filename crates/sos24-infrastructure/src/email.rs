use sendgrid::v3::{Content, Email, Message, Personalization};
use sos24_use_case::shared::adapter::email::{self, EmailSender, SendEmailCommand};

use crate::shared::sendgrid::SendGrid;

pub struct SendGridEmailSender {
    sender: SendGrid,
}

trait EmailToSendGridEmail {
    fn into_email<'a>(&'a self) -> Email;
}

impl EmailToSendGridEmail for email::Email {
    fn into_email<'a>(&'a self) -> Email {
        Email::new(&self.address).set_name(&self.name)
    }
}

impl EmailToSendGridEmail for String {
    fn into_email<'a>(&'a self) -> Email {
        Email::new(self)
    }
}

impl SendGridEmailSender {
    pub fn new(sender: SendGrid) -> Self {
        Self { sender }
    }
}

impl EmailSender for SendGridEmailSender {
    async fn send_email(&self, command: SendEmailCommand) -> anyhow::Result<()> {
        tracing::info!(
            "メールを送信します: subject = {}, len(to) = {}",
            command.subject,
            command.to.len()
        );

        if command.to.is_empty() {
            tracing::info!("宛先が空のためメールを送信しませんでした");
            return Ok(());
        }

        let mut message = Message::new(command.from.into_email())
            .set_subject(&command.subject)
            .add_content(
                Content::new()
                    .set_content_type("text/plain")
                    .set_value(&command.body),
            )
            .add_category("sos");
        if let Some(ref reply_to) = command.reply_to {
            message = message.set_reply_to(reply_to.into_email());
        }

        // 宛先数が1000件より多い場合は分割して送信する必要がある
        // ref: https://sendgrid.kke.co.jp/docs/API_Reference/Web_API_v3/Mail/index.html#-Limitations
        if command.to.len() >= 1000 {
            return Err(anyhow::anyhow!("宛先数が1000件未満である必要があります"));
        }

        for address in &command.to {
            message = message.add_personalization(Personalization::new(address.into_email()));
        }

        self.sender.send(&message).await?;

        tracing::info!("メールを送信しました");
        Ok(())
    }
}
