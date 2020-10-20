/**
 * Trait that is used to show errors that the common user should be able to see
*/
pub trait PublicErrorTrait {
    fn show_public_error(&self) -> String;
}