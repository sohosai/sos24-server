use mockall::automock;

pub struct SendEmailCommand {
    pub from: Email,
    // 1つの宛先につき1通のメールを送信する
    pub to: Vec<String>,
    pub reply_to: Option<String>,
    pub subject: String,
    pub body: String,
}

pub struct Email {
    pub address: String,
    pub name: String,
}

#[automock]
#[allow(async_fn_in_trait)]
pub trait EmailSender: Send + Sync + 'static {
    fn opt_out_url(&self) -> String;
    async fn send_email(&self, command: SendEmailCommand) -> anyhow::Result<()>;
}
