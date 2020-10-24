use lettre_email::EmailBuilder;

#[derive(Clone)]
pub struct Report {
    recipient : String,
    alias : String,
    subject : String,
    message : String
}

impl Report {
    pub fn new(recipient_email : &str, alias_name : &str, sub : &str, msg : &str) -> Result<Self, &'static str> {
        if !crate::util::control_email(recipient_email) {
            warn!("Email does not have the correct format");
            return Err("Email does not have the correct format")
        }
        if sub.is_empty() {
            warn!("Subject of the email is empty");
            return Err("Subject of the email is empty")
        }
        Ok(
            Report {
                recipient : recipient_email.to_owned(),
                alias : alias_name.to_owned(),
                subject : sub.to_owned(),
                message : msg.to_owned()
            }
        )
    }

    // reference getter for the recipient property
    pub fn get_recipient(&self) -> &str { &self.recipient }

    // reference getter for the alias property
    pub fn get_alias(&self) -> &str { &self.alias }

    // reference getter for the subject property
    pub fn get_subject(&self) -> &str { &self.subject }

    // reference getter for the message property
    pub fn get_message(&self) -> &str { &self.message }

    pub fn email(&self) -> EmailBuilder {
        EmailBuilder::new()
        .to((self.get_recipient(), self.get_alias()))
        .subject(self.get_subject())
        .body(self.get_message())
    }
}