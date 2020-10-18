use xam_xam_dal::diesel::PgConnection;
use jwt_gang::claim_config::ClaimConfiguration;
use xam_xam_dal::repo::user;
use mailgang::curl_mail::Mailer;
use redis::Connection;
use crate::err::XamXamServiceError;
use crate::viewmodels::new_user::NewUser;
use xam_xam_dal::models::user::InsertableUser;