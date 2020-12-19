use lettre::{ SmtpTransport, Transport };
use lettre::smtp::authentication::Credentials;
use std::sync::Mutex;
use xam_xam_common::util::get_value_from_key;
use crate::mail_struct::Report;

pub type MailTransport = Mutex<SmtpTransport>;

pub struct Mailer(MailTransport,String);

fn get_transport(domain : &str, email : &str, pwd : &str) -> MailTransport {
    info!("Transport has been created.");
    Mutex::new(
        lettre::SmtpClient::new_simple(&domain.to_string())
        .expect("stmp domain is not good")
        .credentials(Credentials::new(email.to_string(),pwd.to_string()))
        .transport()
    )
}

impl Default for Mailer {
    /**
     * Returns a mailer, based on information detailed in the .env config file. Following lines should be in:
     * PERSON_SMTP_USERNAME: email used to send the email
     * PERSON_SMTP_PASSWORD: password of the user email 
     * PERSON_SMTP_DOMAIN: domain where the email is from
     */
    fn default() -> Self {
        let email = get_value_from_key("PERSON_SMTP_USERNAME").expect("PERSON_SMTP_USERNAME variable not found in the .env config file or as environment variable");
        let password = get_value_from_key("PERSON_SMTP_PASSWORD").expect("PERSON_SMTP_PASSWORD variable not found in the .env config file or as environment variable");
        let domain = get_value_from_key("PERSON_SMTP_DOMAIN")
        .expect("PERSON_SMTP_DOMAIN variable not found in the .env config file or as environment variable");
        Mailer(
            get_transport(&domain, &email, &password),
            email.to_owned()
        )
    }
}

impl Mailer {
    pub fn new(domain : &str, email : &str, pwd : &str) -> Mailer {
        Mailer(
            get_transport(domain, email, pwd),
            email.to_owned()
        )
    }

    pub fn test_mail_to_self(&self) -> Result<(),&'static str> {
        self.send_email(Report::new(&self.1.clone(), "", "Test mail", "Server test")?)
    }

    pub fn send_email(&self, report : Report) -> Result<(),&'static str> {
        let mail = match report.email().from(self.1.clone()).build() {
            Ok(mail) => mail,
            Err(err) => {
                warn!("Faulthy report structure. error: {}",err);
                return Err("Faulthy report structure.")
            }
        };
        match self.0.lock().expect("Could not lock the email transport").send(mail.into()) {
            Ok(_) => {
                info!("Email has been sent through the SMTP transport bus.");
                Ok(())
            },
            Err(err) => {
                warn!("The email could not be send this is related to the smtp transport. error: {}",err);
                Err("The email could not be send this is related to the smtp transport")
            }
        }
    }
}